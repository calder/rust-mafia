use std::fs::File;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::io::{BufReader, Lines};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

use mafia::{Action, Game, Map, Player, Set};

use crate::auth::{Entity, KeyMap};

pub type ConnMap = Map<Player, Set<mpsc::Receiver<Response>>>;

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
    conns: ConnMap,

    /// Game state.
    game: Game,

    /// Authentication keys.
    keys: KeyMap,

    /// Game directory.
    path: PathBuf,
}

/// A single client connection.
struct ServerConn {
    /// Shared server state.
    state: Arc<RwLock<ServerState>>,

    /// Client address
    peer: SocketAddr,

    /// Client reader.
    reader: Lines<BufReader<OwnedReadHalf>>,

    /// Client writer.
    writer: OwnedWriteHalf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Request {
    Auth(String),
    EndPhase,
    Use(Action),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Response {
    Authenticated(Entity),
    Error(String),
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
                conns: ConnMap::new(),
                game: game,
                keys: keys,
                path: path,
            })),
        })
    }

    pub async fn run(self: &mut Self) -> Result<Server, io::Error> {
        loop {
            let (conn, peer) = self.listener.accept().await.unwrap();
            let conn = ServerConn::new(self.state.clone(), conn, peer);

            tokio::spawn(async move {
                conn.run().await.unwrap();
            });
        }
    }
}

impl ServerConn {
    fn new(state: Arc<RwLock<ServerState>>, conn: TcpStream, peer: SocketAddr) -> Self {
        let (reader, writer) = conn.into_split();
        let reader = tokio::io::BufReader::new(reader).lines();

        ServerConn {
            state: state,
            peer: peer,
            reader: reader,
            writer: writer,
        }
    }

    async fn run(mut self: Self) -> Result<(), io::Error> {
        debug!("{}: <CONNECTED>", self.peer);
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
        match request {
            Request::Auth(key) => {
                match self.state.clone().read().await.keys.get(&key) {
                    Some(entity) => {
                        // PLACEHOLDER
                        self.write(Response::Authenticated(entity.clone())).await?;
                    }
                    None => {
                        self.write(Response::Error("Invalid token".to_string()))
                            .await?;
                    }
                }
            }
            Request::EndPhase => {}
            Request::Use(_action) => {}
        };

        Ok(())
    }

    async fn write(self: &mut Self, response: Response) -> Result<(), io::Error> {
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
