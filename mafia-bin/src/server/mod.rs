use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::prelude::*;

pub struct Server {
    listener: TcpListener,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Metadata {
    pub address: std::net::IpAddr,
    pub pid: u32,
    pub port: u16,
}

impl Server {
    pub async fn new(
        address: &str,
        metadata_path: Option<std::path::PathBuf>,
    ) -> Result<Server, io::Error> {
        let listener = TcpListener::bind(address).await?;

        let addr = listener.local_addr().unwrap();
        info!("Listening on {}", addr);

        // Create metadata file.
        //
        // Write to a temporary file then move to the metadata file so watchers
        // can't read while we're part way through writing.
        if let Some(path) = metadata_path {
            let metadata = Metadata {
                address: addr.ip(),
                pid: std::process::id(),
                port: addr.port(),
            };

            let basename = path.file_name().unwrap().to_str().unwrap().to_string();
            let tmp_path = path.with_file_name(basename + ".tmp");
            let tmp_file = std::fs::File::create(tmp_path.clone()).unwrap();
            serde_yaml::to_writer(tmp_file, &metadata).unwrap();
            std::fs::rename(tmp_path, path).unwrap();
        }

        Ok(Server { listener: listener })
    }

    pub async fn run(self: &mut Self) -> Result<Server, io::Error> {
        loop {
            let (mut conn, _) = self.listener.accept().await.unwrap();
            debug!("Client connected.");

            tokio::spawn(async move {
                let (reader, mut writer) = conn.split();
                let mut lines = tokio::io::BufReader::new(reader).lines();
                loop {
                    match lines.next_line().await {
                        Ok(None) => {}
                        Ok(Some(msg)) => {
                            debug!("Received: {:?}", msg);
                            writer.write(msg.to_uppercase().as_bytes()).await.unwrap();
                        }
                        Err(e) => {
                            debug!("Error: {:?}", e);
                            break;
                        }
                    }
                }
                debug!("Client disconnected.");
            });
        }
    }
}
