use mafia_bin::*;

#[tokio::test]
async fn test_server_smoketest() {
    tokio::spawn(main(Mafia {
        cmd: Command::Host { smoketest: true },
    }))
    .await
    .unwrap();
}
