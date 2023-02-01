use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::{Command, ExitCode};
use std::{env, str};
use structopt::StructOpt;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    firefox_binary_path: Option<String>,
    profiles: Vec<Profile>,
}

#[derive(Deserialize)]
struct Profile {
    firefox_container: String,
    aws_vault_profile: String,
}

fn get_config_file(file_path: String) -> Result<Config, String> {
    let file = File::open(&file_path);
    match file {
        Ok(mut k) => {
            let mut contents = String::new();
            let result = k.read_to_string(&mut contents);
            if result.is_err() {
                return Err("Unable to read file as string. ".to_owned());
            }
            let maybe_config: Result<Config, toml::de::Error> = toml::from_str(&contents);
            match maybe_config {
                Ok(config) => return Ok(config),
                Err(e) => {
                    let mut err = String::new();
                    err.push_str("Unable parse toml file. Please see following error.");
                    err.push_str(&format!("File located at {}", file_path));
                    err.push_str(&format!("{}", e));
                    return Err("Unable to parse toml file.".to_owned());
                }
            }
        }
        Err(_) => return Err("Unable to read toml file.".to_owned()),
    }
}

fn find_aws_vault_profile<'a>(
    profiles: &'a Vec<Profile>,
    av_profile: &String,
) -> Option<&'a Profile> {
    for val in profiles.iter() {
        if val.aws_vault_profile == *av_profile {
            return Some(val);
        }
    }
    return None;
}

fn run_firefox_url_in_container(firefox_binary: &String, container: &String, url: &String) {
    let mut arg = "ext+container:name=".to_owned();
    arg.push_str(container);
    arg.push_str("&url=");
    arg.push_str(url);

    let res = Command::new(firefox_binary).arg(arg).output();

    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            println!("totally errored");
        }
    }
}

fn get_login_url(profile: &String) -> Result<String, String> {
    let res = Command::new("aws-vault")
        .args(["login", "--stdout", profile])
        .output();
    match res {
        Ok(w) => match str::from_utf8(&w.stdout) {
            Ok(k) => {
                println!("pre trim {}", k);
                println!("post trim {}", k.trim());
                let trimmed = k.trim().replace("&", "%26");
                return Ok(trimmed.to_owned());
            }
            Err(_) => {
                return Err("Unable to cast to string from vec<u8>".to_owned());
            }
        },
        Err(_e) => return Err("Unable to start command".to_owned()),
    }
}

fn login(profiles: Vec<Profile>, av_profile: &String, ff_bin: &String) -> ExitCode {
    let av_profile = "customs-stage".to_owned();
    match find_aws_vault_profile(&profiles, &av_profile) {
        Some(k) => {
            let login_url = get_login_url(&k.aws_vault_profile);
            if login_url.is_err() {
                println!("Unable to acquire login url");
                return ExitCode::from(1);
            }
            run_firefox_url_in_container(&ff_bin, &k.firefox_container, &login_url.unwrap());
            println!("found lol");
        }
        None => {
            println!("not found lol");
        }
    }
    return ExitCode::from(0);
}

fn list(profiles: Vec<Profile>) -> ExitCode {
    for (i, profile) in profiles.iter().enumerate() {
        println!(
            "{}. aws-vault profile: {}, FF container: {}",
            i + 1,
            profile.aws_vault_profile,
            profile.firefox_container
        )
    }
    return ExitCode::from(0);
}

fn read_config_file_from_different_locations() -> Result<Config, String> {
    let filename = "config.toml";
    let non_xdg_path = format!("$HOME/.config/fav/{}", &filename);
    match env::var("XDG_CONFIG_HOME") {
        Ok(a) => {
            return get_config_file(format!("{}/{}", &a, &filename));
        }
        Err(b) => {}
    }
    if Path::new(&non_xdg_path).exists() {
        return get_config_file(non_xdg_path);
    }
    if Path::new(&filename).exists() {
        return get_config_file(filename.to_owned());
    }

    return Err(
        "Unable to find any configuration files. See documentation in github readme".to_owned(),
    );
}

#[derive(StructOpt, PartialEq, Eq)]
enum SubCommand {
    #[structopt(name = "login")]
    Login {
        #[structopt(long = "profile", short = "p")]
        profile: Option<String>,
        #[structopt(long = "container", short = "c")]
        container: Option<String>,
    },
    List,
    Add,
    Delete,
}

#[derive(StructOpt)]
#[structopt(name = "cli")]
struct Cli {
    #[structopt(subcommand)]
    cmd: SubCommand,
}

fn handle_command(args: Cli, conf: Config) -> ExitCode {
    let ff_bin = &conf.firefox_binary_path.unwrap_or("firefox".to_owned());
    let profiles = conf.profiles;
    match args.cmd {
        SubCommand::Login { profile, container } => {
            println!("Login");
            match profile {
                Some(v) => {
                    return login(profiles, &v, ff_bin);
                }
                None => match container {
                    Some(_c) => {
                        println!("Not implemented");
                        return ExitCode::from(1);
                    }
                    None => {
                        println!("There must be some kind of way out of here");
                        return ExitCode::from(1);
                    }
                },
            }
        }
        SubCommand::List => {
            return list(profiles);
        }
        SubCommand::Add => {
            println!("Add");
        }
        SubCommand::Delete => {
            println!("Delete");
        }
    }
    return ExitCode::from(0);
}

fn main() -> ExitCode {
    let args = Cli::from_args();
    let config_file = read_config_file_from_different_locations();

    match config_file {
        Ok(conf_file) => {
            return handle_command(args, conf_file);
        }
        Err(e) => {
            println!("{}", e);
            return ExitCode::from(1);
        }
    }
}
