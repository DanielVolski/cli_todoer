extern crate getopts;

use std::env;
//use std::fs::File;
//use std::io::prelude::*;
use getopts::Options;

fn main() {
    let args: Vec<String> = env::args().collect();
    let exe = args[0].clone();

    let mut opts = Options::new();

    opts.optflag("h", "help", "help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }    
    };

    if matches.opt_present("h") {
        print_help(&exe, opts);
        return;
    }
}

fn _create_todo_list(list_name: &str) {
    println!("This should create a file with the name {}.txt", list_name);
}

fn print_help(exe: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", exe);
    print!("{}", opts.usage(&brief));
}
