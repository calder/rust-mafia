mod util;

#[tokio::test]
async fn test_version() {
    util::mafia(&["version"]).await.unwrap();
}
