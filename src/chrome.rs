use std::env;
use std::process::Command;

pub fn get_chrome_binary(bin_from_conf: Option<String>) -> String {
    match env::var("BROWSER") {
        Ok(k) => {
            return k;
        }
        Err(_) => {}
    }

    if cfg!(target_os = "macos") {
        return "open".to_owned();
    }
    return bin_from_conf.unwrap_or("chrome-cli".to_owned());
}

pub fn run_chrome_url_in_profile(container: &String, login_url: &String) {

    let mut binary = "chrome-cli";
    let mut args = String::new();
    if cfg!(target_os = "macos") {
        binary = "open";
        args.push_str("-a \"Google Chrome.app\" -n --args");
    } else {
    }

    args.push_str(" --profile-directory ");
    args.push_str(&container);
    args.push_str(" ");
    args.push_str(&login_url);
    println!("Running chrome cli");
    println!("{}", args);

    let res = Command::new(binary).arg(args).output();
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{}", e);
            println!("totally errored");
        }
    }
}
