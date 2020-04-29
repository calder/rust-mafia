#[macro_use]
extern crate log;

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
        /// Address to bind to.
        #[structopt(long, default_value = "0.0.0.0:0")]
        address: String,

        /// File to write server metadata to on startup.
        #[structopt(long, parse(from_os_str))]
        metadata: Option<std::path::PathBuf>,

        /// Start the server then exit.
        #[structopt(long)]
        smoketest: bool,
    },

    /// Print version and exit.
    Version,
}

pub async fn main(args: Vec<String>) {
    // Parse command line args.
    let opt = Mafia::from_iter(args);

    // Initialize logging.
    match env_logger::builder().try_init() {
        Err(e) => {
            eprintln!("Error initializing logging: {:?}", e);
        }
        Ok(()) => {}
    }

    match opt.cmd {
        Command::Join { smoketest } => {
            let mut app = Client::new().unwrap();

            if smoketest {
                app.draw().unwrap();
            } else {
                app.run().unwrap();
            }
        }

        Command::Host {
            address,
            metadata,
            smoketest,
        } => {
            let mut server = Server::new(&address, metadata).await.unwrap();

            if !smoketest {
                server.run().await.unwrap();
            }
        }

        Command::Version => {
            println!("mafia {}", env!("CARGO_PKG_VERSION"));
        }
    }
}
