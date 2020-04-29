use mafia_bin::*;

#[tokio::test]
async fn test_client_smoketest() {
    tokio::spawn(main(Mafia {
        cmd: Command::Join { smoketest: true },
    }))
    .await
    .unwrap();
}
