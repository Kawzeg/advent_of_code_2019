use intcode::{Intcode, intcode_from_file, run, run_with_io};
use std::io;

fn main() -> io::Result<()> {
    let file = "./resources/input";
    let input = intcode_from_file(file)?;
    let output = run_with_io(input, vec![1]);
    println!("Output is {:?}", output);
    Ok(())
}
