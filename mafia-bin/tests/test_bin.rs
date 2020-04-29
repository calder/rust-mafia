mod util;

#[tokio::test]
async fn test_version() {
    util::main(&["mafia", "version"]).await;
}
