#[tokio::main]
async fn main() {
    match mafia_bin::main(std::env::args().collect()).await {
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
        Ok(()) => {}
    }
}
