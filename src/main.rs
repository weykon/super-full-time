use homedir::get_my_home;
use regex::Regex;
use ssh2::Host;
use ssh2_config::{ParseRule, SshConfig};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::Deref;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    println!("processing");
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

    let filtered_hosts:  Vec<&ssh2_config::Host> = hosts
        .iter()
        .filter(|h| match h.params.host_name {
            Some(ref host_name) => re.is_match(host_name),
            None => false,
        })
        .collect();

    filtered_hosts
        .iter()
        .for_each(|x| println!("{}", x.pattern[0].pattern));

    Ok(())
}
