use mafia_bin::ui::App;

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
    Join {
        /// Render the UI once and exit.
        #[structopt(long)]
        smoketest: bool,
    },

    /// Print version and exit.
    Version {},
}

fn main() {
    let opt = Mafia::from_args();

    match opt.cmd {
        Command::Join { smoketest } => {
            let mut app = App::new().unwrap();

            if smoketest {
                app.draw().unwrap();
            } else {
                app.run().unwrap();
            }
        }
        Command::Version {} => {
            println!("mafia {}", env!("CARGO_PKG_VERSION"));
        }
    }
}
