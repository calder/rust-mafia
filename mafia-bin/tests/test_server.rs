mod util;

#[tokio::test]
async fn test_server_smoketest() {
    let tempdir = tempfile::tempdir().unwrap();
    let path = tempdir.path().to_str().unwrap();
    util::mafia(&["init", "--path", path])
        .await
        .unwrap()
        .unwrap();
    util::mafia(&["host", "--path", path, "--smoketest"])
        .await
        .unwrap()
        .unwrap();
}
