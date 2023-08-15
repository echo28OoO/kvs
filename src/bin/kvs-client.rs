use clap::AppSettings;
use kvs::{KvsClient, Result};
use std::net::SocketAddr;
use std::process::exit;
use structopt::StructOpt;

const DEFAULT_LISTENING_ADDRESS: &str = "127.0.0.1:4000";
const ADDRESS_FORMAT: &str = "IP:PORT";

#[derive(StructOpt, Debug)]
#[structopt(
    name = "kvs-client",
    raw(global_settings = "&[\
                            AppSettings::DisableHelpSubcommand,\
                            AppSettings::VersionlessSubcommands\
                            ]")
)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "get", about = "Get the string value of a given string key")]
    Get {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,
        #[structopt(
            long,
            help = "Sets the server address",
            raw(value_name = "ADDRESS_FORMAT"),
            raw(default_value = "DEFAULT_LISTENING_ADDRESS")
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    #[structopt(name = "set", about = "Set the value of a string key to a string")]
    Set {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,
        #[structopt(name = "VALUE", help = "The string value of the key")]
        value: String,
        #[structopt(
            long,
            help = "Sets the server address",
            raw(value_name = "ADDRESS_FORMAT"),
            raw(default_value = "DEFAULT_LISTENING_ADDRESS")
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
    Remove {
        #[structopt(name = "KEY", help = "A string key")]
        key: String,
        #[structopt(
            long,
            help = "Sets the server address",
            raw(value_name = "ADDRESS_FORMAT"),
            raw(default_value = "DEFAULT_LISTENING_ADDRESS")
            parse(try_from_str)
        )]
        addr: SocketAddr,
    },
}

fn main() {
   let opt = Opt::from_args();
   if let Err(e)  = run(opt) {
       eprintln!("Error: {}", e);
       exit(1);
   }
}

fn run(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Get { key, addr } => {
            let mut client = KvsClient::connect(addr)?;
            let value = client.get(key)?;
            if let Some(value) = value {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        Command::Set { key, value, addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.set(key, value)?;
        }
        Command::Remove { key, addr } => {
            let mut client = KvsClient::connect(addr)?;
            client.remove(key)?;
        }
    }
    Ok(())
}