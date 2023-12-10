use std::io;

use lqth::tick;

fn main() {
    let mut out_buf = io::stdout().lock();
    tick(&mut out_buf).unwrap();
}
