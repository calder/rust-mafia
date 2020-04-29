use structopt::StructOpt;

#[tokio::main]
async fn main() {
    let opt = mafia_bin::Mafia::from_args();
    mafia_bin::main(opt).await;
}
