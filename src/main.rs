#[allow(unused)]

mod util;
use util::dec_to_hex;

fn main() {
    let x = dec_to_hex(255);
    println!("{x}");
}