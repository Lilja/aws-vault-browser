#[macro_use]
extern crate log;

mod chrome;
mod firefox;
mod thread;

use std::io::{self, prelude::*};

use std::process::{Command, ExitCode};
use std::{env, str};
use structopt::StructOpt;

fn get_login_url(aw_profile: &String) -> Result<String, String> {
    let mut command = Command::new("aws-vault");
    command.args(["login", "--stdout", aw_profile]);

    let (status, stdout, _stderr) = thread::run_and_capture(&mut command).unwrap();
    match status.code() {
        Some(code) => {
            if code != 0 {
                debug!("Exit code != 0, = {}", code);
                println!("aws-vault command did not succed");
                return Err("aws-vault command did not succed".to_owned());
            }
        }
        None => todo!(),
    }
    match String::from_utf8(stdout) {
        Ok(i) => {
            debug!("output from aws-vault {}", i);
            return Ok(i.lines().last().unwrap().to_owned());
        }
        Err(_) => {
            debug!("Fett med fel bror")
        }
    }

    return Ok("Wut".to_owned());
}

fn replace_ampersand_with_url_encoded_ampersand(s: &String) -> String {
    return s.replace("&", "%26");
}

fn login(
    aw_profile: &String,
    b_container: &String,
    cli_bin: ConfCliPath,
    browser: String,
) -> ExitCode {
    let ff_bin = firefox::get_ff_binary(cli_bin.ff);
    let maybe_log_url = get_login_url(&aw_profile);
    if maybe_log_url.is_err() {
        return ExitCode::from(1);
    }
    let raw_url = maybe_log_url.unwrap();
    if browser == "firefox" {
        let login_url = replace_ampersand_with_url_encoded_ampersand(&raw_url);
        firefox::run_firefox_url_in_container(&ff_bin, &b_container, &login_url);
    } else if browser == "chrome" {
        panic!("Chrome not supported, it's a mess");
    } else {
        panic!("Unknown browser '{}'", browser);
    }
    return ExitCode::from(0);
}

#[derive(StructOpt, PartialEq, Eq)]
enum SubCommand {
    #[structopt(name = "login")]
    Login {
        #[structopt(long = "profile", short = "p")]
        aw_profile: String,
        #[structopt(long = "container", short = "c")]
        b_container: String,
        #[structopt(short, long, default_value = "firefox")]
        browser: String,
    },
}

#[derive(StructOpt)]
#[structopt(name = "cli")]
struct Cli {
    #[structopt(subcommand)]
    cmd: SubCommand,
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short, long)]
    browser_path: Option<String>,
}

struct ConfCliPath {
    ff: Option<String>,
    // chrome: Option<String>,
}

fn handle_command(args: Cli) -> ExitCode {
    let cli_bin = ConfCliPath {
        ff: args.browser_path.or(Option::Some("firefox".to_owned())),
        // chrome: Some("".to_owned()),
    };
    match args.cmd {
        SubCommand::Login {
            aw_profile,
            b_container,
            browser,
        } => {
            return login(&aw_profile, &b_container, cli_bin, browser);
        }
    }
}

fn main() -> ExitCode {
    let args = Cli::from_args();
    if env::var("RUST_LOG").is_err() {
        if args.debug {
            env::set_var("RUST_LOG", "debug")
        } else {
            env::set_var("RUST_LOG", "error")
        }
    }
    env_logger::init();

    io::stdout().flush().unwrap();

    return handle_command(args);
}
