use std::io::Read;
use ssh2::Channel;

pub fn common(channel: &mut Channel) {
    println!("common");
    channel.exec("ls").unwrap();
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    println!("{}", output);
}

