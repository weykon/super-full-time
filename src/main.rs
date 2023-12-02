use std::io;
mod entry_points;
pub mod thread_ready;

fn main() -> io::Result<()> {
    println!("processing");
    let entry_points = entry_points::check();
    thread_ready::threads_boot(entry_points);
    Ok(())
}
