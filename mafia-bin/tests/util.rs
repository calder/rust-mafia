pub fn mafia(args: &[&str]) -> tokio::task::JoinHandle<()> {
    let args = [["mafia", "--verbose=debug"].to_vec(), args.to_vec()]
        .concat()
        .iter()
        .map(|a| a.to_string())
        .collect();

    tokio::spawn(mafia_bin::main(args))
}
