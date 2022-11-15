use std::{fs, io};
use std::borrow::Cow;
use std::io::BufWriter;
use std::net::TcpStream;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread::sleep;
use std::time::Duration;

use anyhow::{Error, Result};
use clap::{Args, Parser, Subcommand};
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

mod template;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init(Init),
    Test(Test),
}

#[derive(Args)]
struct Init {
    #[arg(short, long)]
    name: String,
}

impl Init {
    fn run(&self) -> Result<()> {
        println!("Init: {}", &self.name);
        fs::create_dir(name.clone())?;
        std::env::set_current_dir(&name)?;

        fs::create_dir("sources")?;
        fs::create_dir("e2etests")?;

        fs::write("Move.toml", template::move_toml(&name))?;
        fs::write("sources/my_module.move", template::source(&name))?;

        /* typescript specific generation */
        fs::write("tsconfig.json", template::ts_config())?;
        fs::write("package.json", template::package_json(&name))?;
        fs::write("e2etests/my_module.ts", template::ts_test())?;

        Command::new("npm")
            .arg("install")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| anyhow::format_err!("npm install failed: {}", e.to_string()))?;

        /* git init */
        let git_result = Command::new("git")
            .arg("init")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .map_err(|e| anyhow::format_err!("git init failed: {}", e.to_string()))?;
        if !git_result.status.success() {
            eprintln!("Failed to automatically initialize a new git repository");
        }

        println!("{} initialized", name);
        Ok(())
    }
}

#[derive(Args)]
struct Test {
    #[arg(short, long)]
    contract: bool,
    #[arg(short, long)]
    full: bool,

    #[arg(long)]
    clear: bool,
}

impl Test {
    fn run(&self) -> Result<()> {
        // // GET RID
        // let dir = std::env::current_dir()?.display().to_string();
        // let path = Path::new(&dir).join("example");
        // assert!(std::env::set_current_dir(&path).is_ok());
        // // END GET RID

        println!("Running tests");
        let dir = std::env::current_dir()?.display().to_string();
        let path = Path::new(&dir).join("Move.toml");
        if !path.exists() {
            println!("Could not find Move.toml in current directory");
            return Ok(());
        }

        if self.clear {
            println!("Clearing test directory");
            let path = Path::new(&dir).join("/test-ledger");
            if path.exists() {
                fs::remove_dir_all(path)?;
            }
        }

        if !Path::new(&format!("{}{}", dir.clone(), "/test-ledger/network.yaml")).exists() {
            println!("test-ledger does not exist");
            fs::create_dir_all("./test-ledger")?;
            let create_test_ledger = Command::new("sui")
                .args(["genesis", "--working-dir", "./test-ledger", "--force"])
                .output()
                .expect("Failed to build validator");
            assert!(create_test_ledger.status.success())
        }

        if !self.contract {
            let test_result = Command::new("sui")
                .arg("move")
                .arg("test")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()
                .map_err(anyhow::Error::from);

            match test_result {
                Ok(exit) => {
                    if !exit.status.success() {
                        std::process::exit(exit.status.code().unwrap());
                    }
                }
                Err(err) => {
                    println!("Failed to run test: {:#}", err);
                    return Err(err);
                }
            }
        }

        if !self.full {
            let sui_validator = run_validator();

            match detect_tcp("127.0.0.1:9000", 30) {
                Ok(()) => {
                    let keys = {
                        let file_content = fs::read_to_string("./test-ledger/sui.keystore").expect("error reading keystore file");
                        file_content
                    };
                    let test_result = run_integration_test(&keys);

                    if let Ok(mut child) = sui_validator {
                        if let Err(err) = child.kill() {
                            println!("Failed to kill subprocess {}: {}", child.id(), err);
                        }
                    }

                    match test_result {
                        Ok(exit) => {
                            if !exit.status.success() {
                                std::process::exit(exit.status.code().unwrap());
                            }
                        }
                        Err(err) => {
                            println!("Failed to run test: {:#}", err);
                            return Err(err);
                        }
                    }
                }
                Err(err) => {
                    println!("Failed to detect validator: {:#}", err);
                    if let Ok(mut child) = sui_validator {
                        if let Err(err) = child.kill() {
                            println!("Failed to kill subprocess {}: {}", child.id(), err);
                        }
                    }
                    return Err(err);
                }
            }
        }


        Ok(())
    }
}

pub fn run_validator() -> Result<Child> {
    let sui_validator = Command::new("sui")
        .arg("start")
        .arg("--network.config")
        .arg("./test-ledger/network.yaml")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start sui");

    Ok(sui_validator)
}


pub fn build() -> Result<String> {
    let build_result = Command::new("sui")
        .arg("move")
        .arg("build")
        .arg("--dump-bytecode-as-base64")
        .output()
        .map_err(anyhow::Error::from);

    Ok(String::from_utf8_lossy(&build_result.unwrap().stdout).to_string())
}

pub fn detect_tcp(proto: &str, max: i32) -> Result<()> {
    let stream = TcpStream::connect(proto);
    if stream.is_err() {
        if max > 0 {
            sleep(Duration::from_secs(1));
            detect_tcp(proto, max - 1)
        } else {
            Ok(())
        }
    } else {
        Ok(())
    }
}

pub fn run_integration_test(keys: &String) -> Result<Output> {
    let test_result = Command::new("npm")
        .arg("test")
        .env("SUI_KEYSTORE", &keys)
        .env("SUI_BUILD", build().unwrap())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .map_err(anyhow::Error::from)?;
    Ok(test_result)
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init(ctx) => {
            ctx.run().unwrap();
        }
        Commands::Test(ctx) => {
            ctx.run().unwrap();
        }
    }
}