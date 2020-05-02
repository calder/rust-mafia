use std::fs::File;
use std::io::Write;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::io::{BufReader, Lines};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::RwLock;

use mafia::{Action, Game, Input, Map, Visibility};

type Connections = Vec<Arc<RwLock<ConnState>>>;
type KeyMap = Map<String, Visibility>;

/// Game server.
pub struct Server {
    /// Listening socket.
    listener: TcpListener,

    /// Server state shared between all connections.
    state: Arc<RwLock<ServerState>>,
}

/// Server state shared between all connections.
struct ServerState {
    /// Client connections.
    conns: Connections,

    /// Game state.
    game: Game,

    /// Authentication keys.
    keys: KeyMap,

    /// Game directory.
    path: PathBuf,
}

/// A single client connection.
struct Conn {
    /// Shared server state.
    server: Arc<RwLock<ServerState>>,

    /// Client address
    peer: SocketAddr,

    /// Client reader.
    reader: Lines<BufReader<OwnedReadHalf>>,

    /// Connection state shared between threads.
    state: Arc<RwLock<ConnState>>,
}

struct ConnState {
    /// Authenticated entity.
    auth: Visibility,

    /// Client address
    peer: SocketAddr,

    /// Client writer.
    writer: OwnedWriteHalf,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Request {
    /// Authenticate to the server.
    Auth(String),

    /// Immediately end the current phase (moderator only).
    EndPhase,

    /// Use an action (player only).
    Use(Action),
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Response {
    /// Successfully authenticated as the given entity.
    Authenticated(Visibility),

    /// Error processing request.
    Error(String),

    /// Request accepted.
    Ok,
}

impl Server {
    pub async fn new(path: PathBuf, address: &str) -> Result<Server, io::Error> {
        // Load or create game file.
        let game_path = path.join("game.ron");
        let setup_path = path.join("setup.ron");
        let game = if game_path.exists() {
            load_file(&game_path)?
        } else if setup_path.exists() {
            Game::new_from_state(load_file(&setup_path)?)
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Missing {}. Run `mafia init` to generate one.",
                    setup_path.display()
                ),
            ));
        };

        // Load key file.
        let keys_path = path.join("auth.ron");
        let keys = if keys_path.exists() {
            load_file(&keys_path)?
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Missing {}. Run `mafia init` to generate one.",
                    keys_path.display()
                ),
            ));
        };

        // Open listening socket.
        let listener = TcpListener::bind(address).await?;
        let addr = listener.local_addr().unwrap();
        info!("Listening on {}", addr);

        Ok(Server {
            listener: listener,
            state: Arc::new(RwLock::new(ServerState {
                conns: Connections::new(),
                game: game,
                keys: keys,
                path: path,
            })),
        })
    }

    pub async fn run(self: &mut Self) -> Result<Server, io::Error> {
        loop {
            let (conn, peer) = self.listener.accept().await.unwrap();
            let conn = Conn::new(self.state.clone(), conn, peer);

            tokio::spawn(async move {
                conn.run().await.unwrap();
            });
        }
    }
}

impl ServerState {
    fn apply(self: &mut Self, input: &Input) {
        self.game.apply(input);
        save_file(&self.path.join("game.ron"), &self.game);
    }
}

impl Conn {
    fn new(server: Arc<RwLock<ServerState>>, conn: TcpStream, peer: SocketAddr) -> Self {
        let (reader, writer) = conn.into_split();
        let reader = BufReader::new(reader).lines();

        Conn {
            server: server,
            peer: peer.clone(),
            reader: reader,
            state: Arc::new(RwLock::new(ConnState {
                peer: peer,
                writer: writer,
                auth: Visibility::Public,
            })),
        }
    }

    async fn run(mut self: Self) -> Result<(), io::Error> {
        debug!("{}: <CONNECTED>", self.peer);
        self.server.write().await.conns.push(self.state.clone());
        loop {
            match self.reader.next_line().await {
                Ok(Some(msg)) => {
                    debug!("{}: > {}", self.peer, msg);
                    let request: Request = ron::de::from_str(&msg).unwrap();
                    self.handle(request).await.unwrap();
                }
                Ok(None) => {
                    debug!("{}: <EOF>", self.peer);
                    break;
                }
                Err(e) => {
                    debug!("{}: <ERROR: {}>", self.peer, e);
                    break;
                }
            }
        }
        debug!("{}: <DISCONNECTED>", self.peer);

        Ok(())
    }

    async fn handle(self: &mut Self, request: Request) -> Result<(), io::Error> {
        let mut state = self.state.write().await;
        match request {
            Request::Auth(key) => match self.server.read().await.keys.get(&key) {
                Some(auth) => {
                    state.auth = auth.clone();
                    state.send(Response::Authenticated(auth.clone())).await?;

                    // TODO: Proof-of-concept pubsub to all clients. Remove
                    std::mem::drop(state);
                    for conn in &self.server.read().await.conns {
                        conn.write().await.send(Response::Ok).await?;
                    }
                }
                None => {
                    state
                        .send(Response::Error("Invalid token".to_string()))
                        .await?;
                }
            },
            Request::EndPhase => match &state.auth {
                Visibility::Moderator => {
                    self.server.write().await.apply(&Input::EndPhase);
                    state.send(Response::Ok).await?;
                }
                _ => {
                    state
                        .send(Response::Error("Permission denied".to_string()))
                        .await?;
                }
            },
            Request::Use(action) => match &state.auth {
                Visibility::Player(player) => {
                    self.server
                        .write()
                        .await
                        .apply(&Input::Use(player.clone(), action));
                    state.send(Response::Ok).await?;
                }
                _ => {
                    state
                        .send(Response::Error("Permission denied".to_string()))
                        .await?;
                }
            },
        };

        Ok(())
    }
}

impl ConnState {
    async fn send(self: &mut Self, response: Response) -> Result<(), io::Error> {
        let msg = ron::ser::to_string(&response).unwrap();
        debug!("{}: < {}", self.peer, msg);
        self.writer.write((msg + "\n").as_bytes()).await?;

        Ok(())
    }
}

fn load_file<T: serde::de::DeserializeOwned>(path: &PathBuf) -> Result<T, io::Error> {
    let file = File::open(path).map_err(|e| {
        io::Error::new(
            e.kind(),
            format!("Error opening {}: {}", &path.display(), e),
        )
    })?;

    let result = ron::de::from_reader(file).map_err(|e| match e {
        ron::de::Error::IoError(_) => io::Error::new(
            io::ErrorKind::Other,
            format!("Error reading {}: {}", &path.display(), e),
        ),
        ron::de::Error::Message(_) => io::Error::new(
            io::ErrorKind::Other,
            format!("Error reading {}: {}", &path.display(), e),
        ),
        ron::de::Error::Parser(_, _) => io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("Error at {}:{}:", &path.display(), e),
        ),
    })?;

    Ok(result)
}

/// Atomically serialize a value to a file.
///
/// Atomicity is achieved by writing to a temporary file then renaming. Renames
/// are atomic on most modern filesystems.
fn save_file<T: serde::ser::Serialize>(path: &PathBuf, value: &T) {
    // Serialize value.
    let config = ron::ser::PrettyConfig::default();
    let output = ron::ser::to_string_pretty(&value, config).unwrap();

    // Write to a temporary file then move to real file for atomicity.
    let tmp_path = PathBuf::from(path.to_str().unwrap().to_string() + ".tmp");
    let mut tmp_file = File::create(tmp_path.clone()).unwrap();
    writeln!(tmp_file, "{}", output).unwrap();
    std::fs::rename(tmp_path, path).unwrap();
}
