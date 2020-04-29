#[tokio::main]
async fn main() {
    mafia_bin::main(std::env::args().collect()).await;
}
