#![feature(slice_group_by)]

use crate::vm::Vm;
use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};

mod vm;

fn main() -> io::Result<()> {
    // TODO: use a crate for better arguments handling
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 2);
    let file = File::open(&args[1])?;
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let mut vm = Vm::new(100_000);
    vm.load(&buf);
    vm.run();
    Ok(())
}
