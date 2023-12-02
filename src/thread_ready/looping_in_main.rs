use std::sync::Mutex;

use std::sync::Arc;

use std::sync::mpsc::{Receiver, Sender};

use super::main_step_or_terminal;

pub(crate) fn loopping_in_main(
    rxs: Vec<(String, Arc<Mutex<Receiver<String>>>)>,
    txs: Vec<(String, Arc<Mutex<Sender<String>>>)>,
) {
    let var_name = 0;
    let mut ellapsed = var_name;
    loop {
        for rx in rxs.iter() {
            match rx.1.lock().unwrap().try_recv() {
                Ok(msg) => {
                    println!("这是主线程收到的消息: 从 {} 发出的", rx.0);
                    println!("MSG ::: {}", msg);
                }
                Err(_) => continue,
            }
        }
        // 主线程下的休眠 与 结束判断
        if main_step_or_terminal::step_done_or_terminal(&mut ellapsed, &txs) {
            break;
        }
    }
}
