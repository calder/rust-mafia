extern crate lazy_static;
extern crate log;

mod util;

#[tokio::test]
async fn test_server_smoketest() {
    util::mafia(&["host", "--smoketest"]).await.unwrap();
}

#[tokio::test]
async fn test_server_hello() {
    util::run_server_test("test_server_hello").await;
}
