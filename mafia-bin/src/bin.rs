use structopt::StructOpt;

use mafia_bin::client::Client;
use mafia_bin::server::Server;

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

    /// Host a game.
    Host {
        /// Start the server then exit.
        #[structopt(long)]
        smoketest: bool,
    },

    /// Print version and exit.
    Version {},
}

#[tokio::main]
async fn main() {
    let opt = Mafia::from_args();

    match opt.cmd {
        Command::Join { smoketest } => {
            let mut app = Client::new().unwrap();

            if smoketest {
                app.draw().unwrap();
            } else {
                app.run().unwrap();
            }
        }

        Command::Host { smoketest } => {
            let mut server = Server::new().await.unwrap();

            if !smoketest {
                server.run().await.unwrap();
            }
        }

        Command::Version {} => {
            println!("mafia {}", env!("CARGO_PKG_VERSION"));
        }
    }
}
