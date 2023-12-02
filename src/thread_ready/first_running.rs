use std::io::Read;

pub(crate) fn run( channel: &mut ssh2::Channel) {
    let mut output = String::new();
    // 连接的第一个执行命令
    channel.exec("uname -a").unwrap();
    channel.read_to_string(&mut output).unwrap();
    // println!("{}", output);

    // output.clear();
    
    // channel.exec("last | head -n 5").unwrap();
    // // 读取返回的数据
    // channel.read_to_string(&mut output).unwrap();
    println!("{}", output);
}

