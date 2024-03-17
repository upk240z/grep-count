mod grep_counter;

use std::{env, process};
use getopt::Opt;
use crate::grep_counter::GrepCounter;

fn lpad(str: &str, len: usize, c: char) -> String {
    format!("{}{}", c.to_string().repeat(len - str.len()), str)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let usage = format!("Usage {} -e [regular expression] [FILE]...", args[0]);
    let mut opts = getopt::Parser::new(&args, "e:");
    let mut expression: String = "".to_string();

    loop {
        if let Ok(option) = opts.next().transpose() {
            match option {
                None => break,
                Some(opt) => match opt {
                    Opt('e', Some(s)) => {
                        expression = s;
                    },
                    _ => unreachable!(),
                }
            }
        } else {
            eprintln!("{}", usage);
            process::exit(0);
        }
    }

    if expression.len() == 0 {
        eprintln!("{}", usage);
        process::exit(0);
    }

    let files = args.split_off(opts.index());
    if files.len() == 0 {
        eprintln!("{}", usage);
        process::exit(0);
    }

    let mut counter = GrepCounter::new(&expression, files);
    let collected = counter.scan();

    let mut max_len = 6;
    for (key, _val) in collected {
        if max_len < key.len() {
            max_len = key.len();
        }
    }

    println!("{}: ----------", lpad(" files", max_len, '-'));

    for (key, val) in collected {
        println!("{}: {}", lpad(key, max_len, ' '), val);
    }
}
