extern crate toml;

use std::{env, process};
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs::File;

static OVERRIDES_PATH : &'static str = ".multirust/overrides";
static SETTINGS_PATH : &'static str = ".multirust/settings.toml";

fn with_date<'a>(short: &'a str, toolchain: &'a str) -> Option<&'a str> {
    let date_start = short.len() + 1;
    let date_end   = short.len() + 3 + 4 + 2 + 2;
    let char_range = toolchain.chars()
        .skip(date_start)
        .take(4)
        .all(char::is_numeric);

    if char_range {
        Some(&toolchain[0..date_end])
    } else {
        None
    }
}

fn clean_toolchain_name(toolchain: &str) -> &str {
    static SHORTNAMES : &'static [&'static str] = &["stable", "nightly", "beta"];

    for short in SHORTNAMES {
        if toolchain.starts_with(short) {
            return match with_date(short, toolchain) {
                Some(s) => s,
                None => short
            }
        }
    }

    toolchain
}

fn plain_overrides_file(f: File) {
    let overrides = BufReader::new(f);

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

fn settings_toml(mut settings: File) {
    let mut content = String::new();
    settings.read_to_string(&mut content).expect("Can't read settings file");

    let toml = match toml::Parser::new(&content).parse() {
        Some(table) => table,
        None => {
            println!("default");
            process::exit(0);
        }
    };

    let overrides = match toml.get("overrides") {
        Some(overrides) => overrides,
        None => {
            println!("default");
            process::exit(0);
        }
    };

    let overrides = match overrides.as_table() {
        Some(overrides) => overrides,
        None => {
            println!("default");
            process::exit(0);
        }
    };

    let cwd = env::current_dir().expect("No valid working directory");
    let cwd = format!("{}", cwd.display());

    match overrides.get(&cwd) {
        Some(toolchain) => {
            let toolchain = toolchain.as_str().expect("Toolchain should be a string, it wasn't.");
            println!("{}", clean_toolchain_name(toolchain))
        },
        None => println!("default"),
    }
}

fn main() {
    let home = env::home_dir().expect("Impossible to get your home dir!");

    let mut overrides_path = home.clone();
    overrides_path.push(OVERRIDES_PATH);

    let mut settings_path = home.clone();
    settings_path.push(SETTINGS_PATH);

    match File::open(&overrides_path) {
        Ok(f) => {
            plain_overrides_file(f);
            process::exit(0);
        },
        Err(ref e) if e.kind() == ErrorKind::NotFound => { /* ignored */ },
        Err(_) => {
            println!("default");
            process::exit(0);
        }
    }

    if let Ok(f) = File::open(&settings_path) {
        settings_toml(f);
        process::exit(0);
    }

    println!("default");
}
