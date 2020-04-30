mod util;

#[tokio::test]
async fn test_client_smoketest() {
    util::mafia(&["join", "--smoketest"])
        .await
        .unwrap()
        .unwrap();
}
