use ssh2::DisconnectCode::ByApplication;

use super::{first_running, ThreadBoot};
use std::{
    io::Read,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

pub(crate) fn operations(thread_boot_unit: ThreadBoot, finished_threads: Arc<AtomicUsize>) {
    let session = thread_boot_unit.session;
    let mark_server_name = thread_boot_unit.name_mark;

    // 打开通道
    let mut channel = session.channel_session().unwrap();
    let rx = thread_boot_unit.command_rx;
    let tx = thread_boot_unit.command_tx;

    match first_running::run(&session) {
        Ok(_) => println!("{}: first_running::run success", mark_server_name),
        Err(e) => {
            println!("{}: first_running::run error: {}", mark_server_name, e);
        }
    }

    // looping running...
    loop {
        let command = match rx.lock().unwrap().try_recv() {
            Ok(cmd) => cmd,
            Err(_) => continue,
        };
        match &*command {
            "EXIT" => {
                println!("来自线程下的::  EXIT cmd recv :: {}", mark_server_name);
                // 关闭通道
                channel.send_eof().unwrap();
                channel.close().unwrap();
                channel.wait_close().unwrap();
                println!("Exiting: {}", mark_server_name);
                println!("正在去尝试关闭session");
                session
                    .disconnect(Some(ByApplication), "EXIT cmd recv", None)
                    .unwrap();
                println!("已经关闭session");
                finished_threads.fetch_add(1, Ordering::SeqCst);
                break;
            }
            _ => {
                // 执行命令
                channel.exec(&command).unwrap();
                // 读取返回的数据
                let mut output = String::new();
                channel.read_to_string(&mut output).unwrap();
                println!("{}", output);
                tx.lock().unwrap().send("DONE".to_owned()).unwrap();
            }
        }
    }
}
