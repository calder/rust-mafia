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

use mafia::{Action, Event, Game, Input, Map, Player, PlayerStatus, Visibility};

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

    /// An in-game event.
    Event(Event),

    /// Player status reminder.
    Players(Map<Player, PlayerStatus>),
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
    async fn apply(self: &mut Self, input: &Input) -> Result<(), io::Error> {
        // Update game state.
        let log = self.game.apply(input).to_vec();

        // Save game before generating any externally visible side effects.
        save_file(&self.path.join("game.ron"), &self.game);

        // Distribute log messages and prune dead connections.
        let mut new_conns = Vec::new();
        for conn in &self.conns {
            let mut c = conn.write().await;
            match c.send_updates(&self.game, &log).await {
                Err(e) => {
                    debug!("{} [{:?}]: <ERROR: {}>", c.peer, c.auth, e);
                    // TODO: Will this ever fail with client still connected? If
                    // so, drop so we don't silently stop sending them updates.
                }
                Ok(()) => {
                    new_conns.push(conn.clone());
                }
            }
        }
        self.conns = new_conns;

        Ok(())
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
        let mut auth = Visibility::Public;
        debug!("{} [{:?}]: <CONNECTED>", self.peer, auth);

        // Distribute log.
        {
            // Lock server to prevent new messages from interleaving.
            let mut server = self.server.write().await;
            self.state
                .write()
                .await
                .send_updates(&server.game, &server.game.log)
                .await?;
            server.conns.push(self.state.clone());
        }

        loop {
            match self.reader.next_line().await {
                Ok(Some(msg)) => {
                    debug!("{} [{:?}]: > {}", self.peer, auth, msg);
                    let request: Request = match ron::de::from_str(&msg) {
                        Err(e) => {
                            self.state
                                .write()
                                .await
                                .send(Response::Error(e.to_string()))
                                .await?;
                            continue;
                        }
                        Ok(r) => r,
                    };
                    self.handle(request).await?;
                }
                Ok(None) => {
                    debug!("{} [{:?}]: <EOF>", self.peer, auth);
                    break;
                }
                Err(e) => {
                    debug!("{} [{:?}]: <ERROR: {}>", self.peer, auth, e);
                    break;
                }
            }
            auth = self.state.read().await.auth.clone();
        }
        debug!("{} [{:?}]: <DISCONNECTED>", self.peer, auth);

        Ok(())
    }

    async fn handle(self: &mut Self, request: Request) -> Result<(), io::Error> {
        let mut state = self.state.write().await;
        let auth = state.auth.clone();
        match request {
            Request::Auth(key) => match self.server.read().await.keys.get(&key) {
                Some(auth) => {
                    state.auth = auth.clone();
                    state.send(Response::Authenticated(auth.clone())).await?;
                }
                None => {
                    state
                        .send(Response::Error("Invalid token".to_string()))
                        .await?;
                }
            },
            Request::EndPhase => match auth {
                Visibility::Moderator => {
                    std::mem::drop(state);
                    self.server.write().await.apply(&Input::EndPhase).await?;
                }
                _ => {
                    state
                        .send(Response::Error("Permission denied".to_string()))
                        .await?;
                }
            },
            Request::Use(action) => match auth {
                Visibility::Player(player) => {
                    std::mem::drop(state);
                    self.server
                        .write()
                        .await
                        .apply(&Input::Use(player.clone(), action))
                        .await?;
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
    async fn send(self: &mut Self, message: Response) -> Result<(), io::Error> {
        match message {
            Response::Event(e) => self.send_raw(e).await,
            m => self.send_raw(m).await,
        }
    }

    async fn send_updates(
        self: &mut Self,
        game: &Game,
        updates: &[(Visibility, Event)],
    ) -> Result<(), io::Error> {
        let mut send_players = false;

        for (visibility, event) in updates {
            let visible = match (visibility, &self.auth) {
                (Visibility::Public, _) => true,
                (Visibility::Player(p1), auth) => match auth {
                    Visibility::Moderator => true,
                    Visibility::Player(p2) => p1 == p2,
                    Visibility::Public => false,
                },
                (Visibility::Moderator, auth) => match auth {
                    Visibility::Moderator => true,
                    Visibility::Player(_) => false,
                    Visibility::Public => false,
                },
            };

            if visible {
                self.send(Response::Event(event.clone())).await?;
            }

            send_players |= match event {
                Event::PhaseBegan(_) => true,
                Event::Died(_) => true,
                _ => false,
            };
        }

        if send_players {
            self.send(Response::Players(game.get_statuses())).await?;
        }

        Ok(())
    }

    async fn send_raw<T: serde::ser::Serialize>(
        self: &mut Self,
        message: T,
    ) -> Result<(), io::Error> {
        let msg = ron::ser::to_string(&message).unwrap();
        debug!("{} [{:?}]: < {}", self.peer, self.auth, msg);
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
    let output = ron::ser::to_string_pretty(&value, mafia::ron_pretty_config()).unwrap();

    let tmp_path = PathBuf::from(path.to_str().unwrap().to_string() + ".tmp");
    let mut tmp_file = File::create(tmp_path.clone()).unwrap();
    writeln!(tmp_file, "{}", output).unwrap();

    std::fs::rename(tmp_path, path).unwrap();
}
