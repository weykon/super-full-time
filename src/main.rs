use std::io;
mod connect;
mod entry_points;

fn main() -> io::Result<()> {
    println!("processing");
    let entry_points = entry_points::check();
    connect::threads_boot(entry_points);
    Ok(())
}
