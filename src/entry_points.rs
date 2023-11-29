use dotenv::dotenv;
use homedir::get_my_home;
use regex::Regex;
use ssh2_config::{ParseRule, SshConfig};
use std::env;
use std::{fs::File, io::BufReader};
pub fn check() -> Vec<ssh2_config::Host> {
    dotenv().ok();

    let mut fixed_servers =  Vec::new();

    for (key, value) in env::vars() {
        match key.find("FIXED_SERVER_") {
            Some(_) => {
                fixed_servers.push(value.to_owned());
            },
            None => {},
        }
    }

    let home = get_my_home().expect("Sorry, can't find your home directory.");

    let config_path = home
        .map(|h| h.join(".ssh").join("config"))
        .expect("Failed to get home directory");

    let mut reader =
        BufReader::new(File::open(config_path).expect("Could not open configuration file"));
    let config = SshConfig::default()
        .parse(&mut reader, ParseRule::STRICT)
        .expect("Failed to parse configuration");

    let re = Regex::new(r"\b\d{1,3}(\.\d{1,3}){3}\b").unwrap();

    let hosts = config.get_hosts();

    let filtered_hosts: Vec<&ssh2_config::Host> = hosts
        .iter()
        .filter(|h| match h.params.host_name {
            Some(ref host_name) => re.is_match(host_name),
            None => false,
        })
        .collect();

    filtered_hosts
        .iter()
        .for_each(|x| println!("{}", x.pattern[0].pattern));

    let mut enter_list = Vec::new();
    filtered_hosts
        .into_iter()
        .filter(|x| {
            fixed_servers
                .iter()
                .any(|server| *server == x.pattern[0].pattern)
        })
        .for_each(|x| enter_list.push(x.to_owned()));

    enter_list
}
