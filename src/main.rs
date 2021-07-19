use app::{my_function};
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(path).unwrap();

    let mut lines = contents.lines();

    println!(
        "{}",
        my_function()
    )
}
