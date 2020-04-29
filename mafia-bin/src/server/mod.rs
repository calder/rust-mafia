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
        // Write to a temporary file and then move to the metadata file so
        // watchers don't accidentally read before we've finished writing.
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

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    match conn.read(&mut buf).await {
                        Ok(0) => return,
                        Ok(n) => {
                            let msg = std::str::from_utf8(&buf[0..n]).unwrap();
                            debug!("Received: {:?}", msg);
                            conn.write(msg.to_uppercase().as_bytes()).await.unwrap();
                        }
                        Err(e) => {
                            debug!("Error: {:?}", e);
                            return;
                        }
                    };
                }
            });
        }
    }
}
