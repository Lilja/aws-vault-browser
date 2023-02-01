use std::{env, fs};
use std::process::Command;

/*
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
*/

pub fn run_chrome_url_in_profile(container: &String, login_url: &String) -> Result<(), String> {
    let mut binary = "chrome-cli";
    if cfg!(target_os = "macos") {
        binary = "open";
    }
    let mut cmd = Command::new(binary);

    let dir_name = format!("/tmp/avb/{}", container);
    let result = fs::create_dir_all(&dir_name);
    if result.is_err() {
        println!("{}", result.unwrap_err());
        return Err(format!("Unable to create directory in {} for google chrome", dir_name));
    }
    let mut mac_os_args: Vec<&str> = vec![];
    if cfg!(target_os = "macos") {
        mac_os_args = vec![
            "-a",
            "Google Chrome.app",
            "-n",
            "--args",
        ];
    }
    let dcd = format!("--disk-cache-dir={}", dir_name);
    let udd = format!("--user-data-dir={}", dir_name);

    let login_url_quoted = format!("{}", &login_url);
    let args: Vec<&str> = vec![/*&pd,*/ "--no-first-run", &dcd, &udd, &login_url_quoted,];
    let all_args = [mac_os_args, args].concat();
    println!("{}", all_args.join(" "));

    let res = cmd.args(all_args).output();

    match res {
        Ok(a) => {
            let k = String::from_utf8(a.stdout);
            let j = String::from_utf8(a.stderr);
            println!("stdout: {}", k.unwrap());
            println!("stderr: {}", j.unwrap());
        }
        Err(e) => {
            println!("{}", e);
            println!("totally errored");
        }
    }
    return Ok(());
}
