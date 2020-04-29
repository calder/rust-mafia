pub async fn main(args: &[&str]) {
    tokio::spawn(mafia_bin::main(
        args.to_vec().iter().map(|a| a.to_string()).collect(),
    ))
    .await
    .unwrap();
}
