mod util;

#[tokio::test]
async fn test_server_smoketest() {
    util::main(&["mafia", "host", "--smoketest"]).await;
}
