use std::io::{self};
mod connect;
mod entry_points;
fn main() -> io::Result<()> {
    println!("processing");
    let entry_points = entry_points::check();
    connect::sshs(entry_points);
    Ok(())
}
