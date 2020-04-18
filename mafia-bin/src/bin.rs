use mafia_bin::ui;

use structopt::StructOpt;

/// Rust implementation of the classic party game Mafia.
#[derive(StructOpt)]
struct Mafia {
    /// Subcommand.
    #[structopt(subcommand)]
    cmd: Command,
}

/// Subcommand.
#[derive(StructOpt)]
enum Command {
    /// Join a game.
    Join {},

    /// Print version and exit.
    Version {},
}

fn main() {
    let opt = Mafia::from_args();

    match opt.cmd {
        Command::Join {} => {
            ui::main().unwrap();
        }
        Command::Version {} => {
            println!("mafia {}", env!("CARGO_PKG_VERSION"));
        }
    }
}
