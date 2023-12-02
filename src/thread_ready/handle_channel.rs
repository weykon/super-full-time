use super::ThreadBoot;
use std::io::Read;

pub(crate) fn operations(thread_boot_unit: ThreadBoot) {
    let session = thread_boot_unit.session;
    let mark_server_name = thread_boot_unit.name_mark;

    // 打开通道
    let mut channel = session.channel_session().unwrap();
    let rx = thread_boot_unit.command_rx;
    let tx = thread_boot_unit.command_tx;

    // looping running...
    loop {
        let command = match rx.recv() {
            Ok(cmd) => cmd,
            Err(e) => break,
        };
        match &*command {
            "EXIT" => {
                // 关闭通道
                channel.send_eof().unwrap();
                channel.wait_close().unwrap();
                println!("Exited: {}", mark_server_name);
            }
            _ => {
                // 执行命令
                channel.exec(&command).unwrap();
                // 读取返回的数据
                let mut output = String::new();
                channel.read_to_string(&mut output).unwrap();
                println!("{}", output);
                tx.send("DONE".to_owned()).unwrap();
            }
        }
    }

    // 连接的第一个执行命令
    channel.exec("uname -a").unwrap();
    channel.exec("last | head -n 5").unwrap();
    // 读取返回的数据
    let mut output = String::new();
    channel.read_to_string(&mut output).unwrap();
    println!("{}", output);
}
