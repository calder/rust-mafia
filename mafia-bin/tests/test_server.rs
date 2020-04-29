#[macro_use]
extern crate log;

use std::fs::File;
use std::time::Duration;

use tokio::prelude::*;

mod util;

#[tokio::test]
async fn test_server_smoketest() {
    util::mafia(&["host", "--smoketest"]).await.unwrap();
}

#[tokio::test]
async fn test_server_hello() {
    let dir = tempfile::tempdir().unwrap();
    let metadata_path = dir.path().join("meta.yaml");
    let metadata_str = metadata_path.to_str().unwrap().to_string();
    let _server = util::mafia(&["host", "--metadata", &metadata_str]);

    while !metadata_path.exists() {
        tokio::time::delay_for(Duration::from_millis(1)).await;
    }
    let metadata_file = File::open(metadata_path).unwrap();
    let metadata: mafia_bin::server::Metadata = serde_yaml::from_reader(metadata_file).unwrap();

    let addr = format!("127.0.0.1:{}", metadata.port);
    let _client = tokio::spawn(async move {
        let mut conn = tokio::net::TcpStream::connect(addr).await.unwrap();
        conn.write(b"Hello world\n").await.unwrap();

        let mut buf = [0; 1024];
        match conn.read(&mut buf).await {
            Ok(0) => return,
            Ok(n) => {
                let msg = std::str::from_utf8(&buf[0..n]).unwrap();
                debug!("Received: {:?}", msg);
            }
            Err(e) => {
                debug!("Error: {:?}", e);
            }
        };
    })
    .await;
    tokio::time::delay_for(Duration::from_millis(1)).await;
}
