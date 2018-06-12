extern crate clap;
extern crate track8_program;
use clap::{App, Arg};
use std::fs::File;
use std::io;
use track8_program::*;
fn main() -> io::Result<()> {
    let m = App::new("8 track programming language")
        .version("1.0")
        .author("Amelorate")
        .about("Programming language upon a 8 track cartridge")
        .arg(
            Arg::with_name("programs")
                .short("p")
                .long("programs")
                .help("Number of allowed programs in one tape. Default: 8")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("stack_size")
                .short("s")
                .long("stacksize")
                .help("Number of allowed values on the stack. Default: 8")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("FILE")
                .help("Program file to use")
                .required(true)
                .index(1),
        )
        .get_matches();
    let f = File::open(m.value_of("FILE").unwrap())?;
    let mut i = I::new();
    i.max_programs = m
        .value_of("programs")
        .unwrap_or("8")
        .parse()
        .expect("non number value for programs");
    i.max_stack = m
        .value_of("stack_size")
        .unwrap_or("8")
        .parse()
        .expect("non number value for stacksize");
    i.rd(f);
    loop {
        match i.step() {
            R::Running => {}
            R::Stopped => break,
        }
    }
    Ok(())
}
