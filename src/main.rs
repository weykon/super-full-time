use homedir::get_my_home;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let home = get_my_home().expect("Sorry, can't find your home directory.");

    let config_path = home
        .map(|h| h.join(".ssh").join("config"))
        .expect("Failed to get home directory");

    let mut reader =
        BufReader::new(File::open(config_path).expect("Could not open configuration file"));
    let config = SshConfig::default()
        .parse(&mut reader, ParseRule::STRICT)
        .expect("Failed to parse configuration");

    // Query attributes for a certain host
    let params = config.

    Ok(())
}
