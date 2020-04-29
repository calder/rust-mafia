mod util;

#[tokio::test]
async fn test_client_smoketest() {
    util::main(&["mafia", "join", "--smoketest"]).await;
}
