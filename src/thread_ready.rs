use ssh2_config::Host;
use std::thread;

mod ssh_ready;

mod handle_channel;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadBoot {
    name_mark: String,
    session: ssh2::Session,
    command_rx: Arc<Mutex<mpsc::Receiver<String>>>,
    command_tx: Arc<Mutex<mpsc::Sender<String>>>,
}

pub fn threads_boot(entry_points: Vec<Host>) {
    let mut handles = vec![];
    let mut txs = vec![];
    let mut rxs = vec![];
    for server in entry_points.iter() {
        let (tx, rx) = mpsc::channel();
        let tx = Arc::new(Mutex::new(tx));
        let rx = Arc::new(Mutex::new(rx));
        let server = server.clone();
        let mark_server_name = server.pattern[0].pattern.to_owned();
        txs.push((mark_server_name.clone(), Arc::clone(&tx)));
        rxs.push((mark_server_name.clone(), Arc::clone(&rx)));
        let handle = thread::Builder::new()
            // 标记线程名称
            .name(mark_server_name.to_owned())
            .spawn(move || {
                let session = match ssh_ready::handle_ssh_connection(server) {
                    Ok(sess) => sess,
                    Err(e) => {
                        println!("handle_ssh_connection error: {:?}", e);
                        return;
                    }
                };

                // 通道内
                handle_channel::operations(ThreadBoot {
                    name_mark: mark_server_name,
                    session,
                    command_rx: Arc::clone(&rx),
                    command_tx: Arc::clone(&tx),
                });
            })
            .unwrap();
        handles.push(handle);
    }

    // 这里的是主线程的循环，来检查来自线程下的消息从而作出的反应
    looping_in_main::loopping_in_main(rxs, txs);

    for handle in handles {
        handle.join().unwrap();
    }

}

mod looping_in_main;
mod first_running;
mod main_step_or_terminal;
