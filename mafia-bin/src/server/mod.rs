use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

use mafia::{Action, Game, Map, Player, Set};

use crate::auth::{Entity, KeyMap};

pub type ConnMap = Map<Player, Set<mpsc::Receiver<Response>>>;

pub struct Server {
    path: PathBuf,
    keys: Arc<RwLock<KeyMap>>,
    game: Game,
    listener: TcpListener,
    conns: Arc<RwLock<ConnMap>>,
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
            path: path,
            keys: Arc::new(RwLock::new(keys)),
            game: game,
            conns: Arc::new(RwLock::new(ConnMap::new())),
            listener: listener,
        })
    }

    pub async fn run(self: &mut Self) -> Result<Server, io::Error> {
        loop {
            let (conn, _) = self.listener.accept().await.unwrap();
            let peer = conn.peer_addr().unwrap();
            debug!("{}: <CONNECTED>", peer);

            let keys = self.keys.clone();
            let conns = self.conns.clone();
            tokio::spawn(async move {
                let (reader, mut writer) = conn.into_split();
                // let (mut tx, mut rx) = mpsc::channel(1);

                let mut lines = tokio::io::BufReader::new(reader).lines();
                loop {
                    match lines.next_line().await {
                        Ok(Some(msg)) => {
                            debug!("{}> {}", peer, msg);
                            let request: Request = ron::de::from_str(&msg).unwrap();
                            handle(request, &mut writer, &peer, &conns, &keys)
                                .await
                                .unwrap();
                        }
                        Ok(None) => {
                            debug!("{}: <EOF>", peer);
                            break;
                        }
                        Err(e) => {
                            debug!("{}: <ERROR: {}>", peer, e);
                            break;
                        }
                    }
                }
                debug!("{}: <DISCONNECTED>", peer);
            });
        }
    }
}

async fn handle(
    request: Request,
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    peer: &std::net::SocketAddr,
    conns: &Arc<RwLock<ConnMap>>,
    keys: &Arc<RwLock<KeyMap>>,
) -> Result<(), io::Error> {
    match request {
        Request::Auth(key) => {
            match keys.read().await.get(&key) {
                Some(entity) => {
                    // PLACEHOLDER
                    write(writer, peer, Response::Authenticated(entity.clone())).await?;
                }
                None => {
                    write(writer, peer, Response::Error("Invalid token".to_string())).await?;
                }
            }
        }
        Request::EndPhase => {}
        Request::Use(_action) => {}
    };

    Ok(())
}

async fn write(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    peer: &std::net::SocketAddr,
    response: Response,
) -> Result<(), io::Error> {
    let msg = ron::ser::to_string(&response).unwrap();
    debug!("{}< {}", peer, msg);
    writer.write((msg + "\n").as_bytes()).await?;

    Ok(())
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
