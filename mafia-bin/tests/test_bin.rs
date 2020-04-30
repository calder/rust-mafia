mod util;

#[tokio::test]
async fn test_version() {
    util::mafia(&["version"]).await.unwrap().unwrap();
}

#[tokio::test]
async fn test_init() {
    let test_dir = "tests/test_init";
    if std::path::Path::new(test_dir).is_dir() {
        std::fs::remove_dir_all(test_dir).unwrap();
    }
    util::mafia(&["init", "--path", test_dir, "--seed=1234567890"])
        .await
        .unwrap()
        .unwrap();
}
