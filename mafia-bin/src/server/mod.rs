use std::fs::File;
use std::path::PathBuf;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

use mafia::{Action, Game, Map, Player, Set};

use crate::auth::KeyMap;

pub type ChanMap = Map<Player, Set<mpsc::Receiver<Response>>>;

pub struct Server {
    chans: Arc<RwLock<ChanMap>>,
    game: Game,
    listener: TcpListener,
    keys: Arc<RwLock<KeyMap>>,
    path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Metadata {
    pub address: std::net::IpAddr,
    pub pid: u32,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Request {
    Auth(String),
    EndPhase,
    Use(Action),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Response {
    Error(String),
}

impl Server {
    pub async fn new(path: PathBuf, address: &str) -> Result<Server, io::Error> {
        // Load or create game file.
        let game_path = path.join("game.ron");
        let setup_path = path.join("setup.ron");
        let game = if game_path.exists() {
            load_file(&game_path)
        } else if setup_path.exists() {
            Game::new_from_state(load_file(&setup_path))
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Missing {}. Run `mafia init` to generate one.",
                    setup_path.display()
                ),
            ));
        };

        let listener = TcpListener::bind(address).await?;

        let addr = listener.local_addr().unwrap();
        info!("Listening on {}", addr);

        Ok(Server {
            chans: Arc::new(RwLock::new(ChanMap::new())),
            game: game,
            listener: listener,
            keys: Arc::new(RwLock::new(KeyMap::new())),
            path: path,
        })
    }

    pub async fn run(self: &mut Self) -> Result<Server, io::Error> {
        loop {
            let (conn, _) = self.listener.accept().await.unwrap();
            let peer = conn.peer_addr().unwrap();
            debug!("{}: <CONNECTED>", peer);

            let keys = self.keys.clone();
            let chans = self.chans.clone();
            tokio::spawn(async move {
                let (reader, mut writer) = conn.into_split();
                // let (mut tx, mut rx) = mpsc::channel(1);

                let mut lines = tokio::io::BufReader::new(reader).lines();
                loop {
                    match lines.next_line().await {
                        Ok(Some(msg)) => {
                            debug!("{}: {}", peer, msg);
                            let request: Request = ron::de::from_str(&msg).unwrap();
                            handle(&request, &mut writer, &chans, &keys).await;
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
    request: &Request,
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    chans: &Arc<RwLock<ChanMap>>,
    keys: &Arc<RwLock<KeyMap>>,
) {
    match request {
        Request::Auth(key) => {
            match keys.read().await.get(key) {
                Some(_player) => {
                    // PLACEHOLDER
                }
                None => {
                    writer
                        .write(
                            (ron::ser::to_string(&Response::Error("Invalid token".to_string()))
                                .unwrap()
                                + "\n")
                                .as_bytes(),
                        )
                        .await
                        .unwrap();
                }
            }
        }
        Request::EndPhase => {}
        Request::Use(_action) => {}
    }
}

fn load_file<T: serde::de::DeserializeOwned>(path: &PathBuf) -> T {
    let file = File::open(path).expect(&format!("Error opening {}", &path.display()));
    ron::de::from_reader(file).expect(&format!("Error reading {}", &path.display()))
}
