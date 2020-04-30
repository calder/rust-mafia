use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio::sync::mpsc;
use tokio::sync::RwLock;

use mafia::{Action, Map, Player, Set};

use crate::auth::KeyMap;

pub type ChanMap = Map<Player, Set<mpsc::Receiver<Response>>>;

pub struct Server {
    chans: Arc<RwLock<ChanMap>>,
    listener: TcpListener,
    keys: Arc<RwLock<KeyMap>>,
    path: std::path::PathBuf,
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
    pub async fn new(path: std::path::PathBuf, address: &str) -> Result<Server, io::Error> {
        let listener = TcpListener::bind(address).await?;

        let addr = listener.local_addr().unwrap();
        info!("Listening on {}", addr);

        Ok(Server {
            listener: listener,
            chans: Arc::new(RwLock::new(ChanMap::new())),
            keys: Arc::new(RwLock::new(KeyMap::new())),
            path: path,
        })
    }

    pub async fn run(self: &mut Self) -> Result<Server, io::Error> {
        loop {
            let (mut conn, _) = self.listener.accept().await.unwrap();
            let peer = conn.peer_addr().unwrap();
            debug!("{}: <CONNECTED>", peer);

            let keys = self.keys.clone();
            let _chans = self.chans.clone();
            tokio::spawn(async move {
                let (reader, mut writer) = conn.split();
                // let (mut tx, mut rx) = mpsc::channel(1);

                let mut lines = tokio::io::BufReader::new(reader).lines();
                loop {
                    match lines.next_line().await {
                        Ok(Some(msg)) => {
                            debug!("{}: {}", peer, msg);
                            let request: Request = ron::de::from_str(&msg).unwrap();
                            match request {
                                Request::Auth(key) => {
                                    match keys.read().await.get(&key) {
                                        Some(_player) => {
                                            // PLACEHOLDER
                                        }
                                        None => {
                                            writer
                                                .write(
                                                    (ron::ser::to_string(&Response::Error(
                                                        "Invalid token".to_string(),
                                                    ))
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
