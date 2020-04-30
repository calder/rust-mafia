pub fn mafia(args: &[&str]) -> tokio::task::JoinHandle<Result<(), std::io::Error>> {
    let args = [["mafia"].to_vec(), args.to_vec()]
        .concat()
        .iter()
        .map(|a| a.to_string())
        .collect();

    tokio::spawn(mafia_bin::main(args))
}
