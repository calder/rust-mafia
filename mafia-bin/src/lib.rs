pub mod client;
pub mod server;

use structopt::StructOpt;

use crate::client::Client;
use crate::server::Server;

/// Rust implementation of the classic party game Mafia.
#[derive(StructOpt)]
pub struct Mafia {
    /// Subcommand.
    #[structopt(subcommand)]
    pub cmd: Command,
}

/// Subcommand.
#[derive(StructOpt)]
pub enum Command {
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

pub async fn main(opt: Mafia) {
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
