use homedir::get_my_home;
use regex::Regex;
use ssh2::Host;
use ssh2_config::{ParseRule, SshConfig};
use std::fs::{set_permissions, File};
use std::io::{self, BufRead, BufReader, Read};
use std::net::TcpStream;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::thread;
mod connect;
mod entry_points;
fn main() -> io::Result<()> {
    println!("processing");
    let entry_points = entry_points::check();
    // connect::sshs(entry_points);
    Ok(())
}
