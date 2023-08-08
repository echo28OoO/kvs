use clap::{App, AppSettings, Arg, SubCommand};
use kvs::{KvStore, KvsError, Result};
use std::env::current_dir;
use std::process::exit;

fn main() -> Result<()> {
    // 使用 clap 进行命令行解析
    // 获取 匹配后的 matches
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION")) // 读取 cargo 配置
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("set") // 设置读取 set 命令
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").help("A string key").required(true)) // 设置第一个参数为 key
                .arg(
                    Arg::with_name("VALUE")
                        .help("The string value of the key")
                        .required(true),
                ), //  设置第二个参数为 value
        )
        .subcommand(
            SubCommand::with_name("get") // 设置读取 get 命令
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)), // 设置第一个参数为 key
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").help("A string key").required(true)),
        )
        .get_matches();

    // 匹配命令做相应的动作
    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();
            let value = matches.value_of("VALUE").unwrap();
            let mut store = KvStore::open(current_dir()?)?;
            store.set(key.to_string(), value.to_string())?;
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();

            let mut store = KvStore::open(current_dir()?)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
         }
        ("rm", Some(matches)) => {
            let key = matches.value_of("KEY").unwrap();

            let mut store = KvStore::open(current_dir()?)?;
            match store.remove(key.to_string()) {
                Ok(()) => {}
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                }
                Err(e) => return Err(e),
            }
        }
        _ => unreachable!(),
    }
    Ok(())
}
