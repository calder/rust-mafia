mod util;

#[tokio::test]
async fn test_server_smoketest() {
    util::mafia(&["host", "--smoketest"]).await.unwrap();
}
