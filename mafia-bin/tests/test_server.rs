// use tokio::prelude::*;

mod util;

#[tokio::test]
async fn test_server_smoketest() {
    util::mafia(&["host", "--smoketest"]).await;
}

#[tokio::test]
async fn test_server_hello() {
    // util::main(&["host"]).await;

    // let client = tokio::spawn(async {
    //     let mut conn = tokio::net::TcpStream::connect("127.0.0.1:8080").await.unwrap();
    //     conn.write(b"hello world\n").await.unwrap();
    // });

    // tokio::try_join!(server, client).unwrap();
}
