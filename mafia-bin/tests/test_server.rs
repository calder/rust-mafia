use std::time::Duration;

// use tokio::prelude::*;

mod util;

#[tokio::test]
async fn test_server_smoketest() {
    util::mafia(&["host", "--smoketest"]).await;
}

#[tokio::test]
async fn test_server_hello() {
    let dir = tempfile::tempdir().unwrap();
    let metadata = dir.path().join("meta.yaml");
    let metadata_str = metadata.to_str().unwrap().to_string();
    let server = util::mafia(&["host", "--metadata", &metadata_str]);

    while !metadata.exists() {
        tokio::time::delay_for(Duration::from_millis(1)).await;
    }

    let client = tokio::spawn(async {
        // let mut conn = tokio::net::TcpStream::connect("127.0.0.1:8080").await.unwrap();
        // conn.write(b"hello world\n").await.unwrap();
    })
    .await;
}
