use std::io;
mod entry_points;
pub mod thread_ready;
mod ui;

fn main() -> io::Result<()> {
    println!("processing");
    ui::main::main();
    // let entry_points = entry_points::check();
    // thread_ready::threads_boot(entry_points);
    Ok(())
}
