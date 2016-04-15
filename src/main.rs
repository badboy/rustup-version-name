use std::{env, process};
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;

static OVERRIDES_PATH : &'static str = ".multirust/overrides";

fn clean_toolchain_name(toolchain: &str) -> &str {
    static SHORTNAMES : &'static [&'static str] = &["stable", "nightly", "beta"];

    for short in SHORTNAMES {
        if toolchain.starts_with(short) {
            return short;
        }
    }

    toolchain
}

fn main() {
    let home = env::home_dir().expect("Impossible to get your home dir!");
    let mut overrides_path = home.clone();
    overrides_path.push(OVERRIDES_PATH);

    let overrides = match File::open(&overrides_path) {
        Ok(f) => f,
        Err(_) => {
            println!("default");
            process::exit(0);
        }
    };
    let overrides = BufReader::new(overrides);

    let mut overrides_map = HashMap::<PathBuf, String>::new();

    for line in overrides.lines() {
        let line = line.expect("No valid line found");
        let mut s = line.split(';');
        let path = s.next().expect("No path in line");
        let toolchain = s.next().expect("No toolchain in line");

        let path = PathBuf::from(path);

        overrides_map.insert(path, toolchain.into());
    }

    let cwd = env::current_dir().expect("No valid working directory");

    match overrides_map.get(&cwd) {
        Some(toolchain) => println!("{}", clean_toolchain_name(toolchain)),
        None => println!("default"),
    }
}
