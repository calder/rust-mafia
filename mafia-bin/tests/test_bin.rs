use mafia_bin::*;

#[tokio::test]
async fn test_version() {
    tokio::spawn(main(Mafia {
        cmd: Command::Version,
    }))
    .await
    .unwrap();
}
