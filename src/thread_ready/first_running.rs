use std::io::Read;

pub(crate) fn run(session: &ssh2::Session) -> Result<(), Box<ssh2::Error>> {
    let mut channel = session.channel_session()?; 
    // 连接的第一个执行命令
    channel.exec("uname -a")?;
    let mut output = String::new();
    let _ = channel.read_to_string(&mut output);
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;
    println!("First command output: {}", output);

    output.clear();

    // 重置通道并执行第二个命令
    channel = session.channel_session()?;
    channel.exec("last | head -n 5")?;
    let _ = channel.read_to_string(&mut output);
    channel.wait_eof()?;
    channel.close()?;
    channel.wait_close()?;
    println!("Second command output: {}", output);
    Ok(())
}
