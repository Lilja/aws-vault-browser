mod firefox;
mod chrome;
mod thread;

use std::fs::File;
use std::io::{prelude::*, self};

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
    container: String,
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
                    err.push_str("Unable parse toml file. Please see following error.\n");
                    err.push_str(&format!("File located at {}\n", file_path));
                    err.push_str(&format!("{}", e));
                    return Err(format!("Unable to parse toml file. {}", err));
                }
            }
        }
        Err(_) => return Err(
            format!("Unable to read toml file at {}", file_path)
        ),
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

fn get_login_url(profile: &String) -> Result<String, String> {
    let mut command = Command::new("aws-vault");
    command.args(["login", "--stdout", profile]);

    let (status, stdout, _stderr) = thread::run_and_capture(&mut command).unwrap();
    match status.code() {
        Some(code) => {
            if code != 0 {
                println!("aws-vault command did not succed");
                return Err("aws-vault command did not succed".to_owned());
            }
        },
        None => todo!(),
    }
    match String::from_utf8(stdout) {
        Ok(i) => {
            println!("blahi blaha: {}", i);
            return Ok(i.lines().last().unwrap().to_owned());
        }
        Err(_) => {
            println!("Fett med fel bror")
        }
    }

    return Ok("Wut".to_owned());
}

fn replace_ampersand_with_url_encoded_ampersand(s: &String) -> String {
    return s.replace("&", "%26");
}

fn login(profiles: Vec<Profile>, av_profile: &String, cli_bin: ConfCliPath, browser: String) -> ExitCode {
    let ff_bin = firefox::get_ff_binary(cli_bin.ff);
    match find_aws_vault_profile(&profiles, &av_profile) {
        Some(k) => {
            let maybe_log_url = get_login_url(&k.aws_vault_profile);
            if maybe_log_url.is_err() {
                return ExitCode::from(1);
            }
            let raw_url = maybe_log_url.unwrap();
            if browser == "firefox" {
                let login_url = replace_ampersand_with_url_encoded_ampersand(&raw_url);
                firefox::run_firefox_url_in_container(&ff_bin, &k.container, &login_url);
            } else if browser == "chrome" {
                match chrome::run_chrome_url_in_profile(&k.container, &raw_url) {
                    Ok(_v) => {}
                    Err(e) => {
                        println!("{}", e);
                        return ExitCode::from(1);
                    }

                }
            } else {
                panic!("Unknown browser '{}'", browser);

            }
        }
        None => {
            println!("Could not find aws-vault profile");
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
            profile.container
        )
    }
    return ExitCode::from(0);
}

fn read_config_file_from_different_locations() -> Result<Config, String> {
    let filename = "config.toml";
    let non_xdg_path = format!("$HOME/.config/avb/{}", &filename);
    match env::var("XDG_CONFIG_HOME") {
        Ok(a) => {
            let xdg_path = format!("{}/avb/{}", &a, &filename);
            let xdg_path_config_exists = Path::new(&xdg_path).exists();
            if xdg_path_config_exists {
                return get_config_file(xdg_path);
            }
        }
        Err(_) => {}
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
        #[structopt(short, long, default_value = "firefox")]
        browser: String,
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

struct ConfCliPath {
    ff: Option<String>,
    // chrome: Option<String>,
}

fn handle_command(args: Cli, conf: Config) -> ExitCode {
    let profiles = conf.profiles;
    let cli_bin =  ConfCliPath {
        ff: conf.firefox_binary_path,
        // chrome: Some("".to_owned()),
    };
    match args.cmd {
        SubCommand::Login { profile, container, browser } => {
            println!("Login");
            match profile {
                Some(v) => {
                    return login(profiles, &v, cli_bin, browser);
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

    io::stdout().flush().unwrap();

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
