use std::time::Duration;

use std::thread;

use std::sync::mpsc::Sender;

use std::sync::Mutex;

use std::sync::Arc;

pub(crate) fn step_done_or_terminal(
    ellapsed: &mut i32,
    txs: &Vec<(String, Arc<Mutex<Sender<String>>>)>,
)  {
    terminal(ellapsed, txs);
    // step的时间段
    thread::sleep(Duration::from_millis(200));
}

// 主线程的step到头检查点
pub(crate) fn terminal(ellapsed: &mut i32, txs: &Vec<(String, Arc<Mutex<Sender<String>>>)>) {
    *ellapsed += 200;
    println!("Ellapsed: {}", *ellapsed);
    if *ellapsed >= 3_000 {
        println!("从主线程 Sending EXIT...");
        for tx in txs.iter() {
            tx.1.lock().unwrap().send("EXIT".to_owned()).unwrap();
        }
    }
}
