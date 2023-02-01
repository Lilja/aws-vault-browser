use std::env;
use std::process::Command;


pub fn get_ff_binary(bin_from_conf: Option<String>) -> String {
    match env::var("BROWSER") {
        Ok(k) => {
            return k;
        }
        Err(_) => {}
    }
    return bin_from_conf.unwrap_or("firefox".to_owned());
}

pub fn run_firefox_url_in_container(firefox_binary: &String, container: &String, url: &String) {
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


