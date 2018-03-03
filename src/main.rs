extern crate toml;

use std::{env, process};
use std::io::prelude::*;
use std::io::{BufReader, ErrorKind};
use std::collections::BTreeMap;
use std::fs::File;

use toml::Value;

static OVERRIDES_PATH : &'static str = ".multirust/overrides";
static SETTINGS_PATH : &'static str = ".rustup/settings.toml";
static OLD_SETTINGS_PATH : &'static str = ".multirust/settings.toml";

enum OverridesDatabase {
    Plain(BTreeMap<String, String>),
    Toml(BTreeMap<String, toml::Value>),
}

impl OverridesDatabase {
    pub fn get(&self, key: &str) -> Option<&str> {
        use OverridesDatabase::*;

        match *self {
            Plain(ref db) => db.get(key).map(|s| &s[..]),
            Toml(ref db) => {
                db.get(key).map(|v| v.as_str().expect("Expected value is not a string."))
            }
        }
    }
}

fn with_date<'a>(short: &'a str, toolchain: &'a str) -> Option<&'a str> {
    let date_start = short.len() + 1;
    let date_end   = short.len() + 3 + 4 + 2 + 2;
    let char_range = toolchain.chars()
        .skip(date_start)
        .take(4)
        .all(char::is_numeric);

    if toolchain.len() > date_start && char_range {
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

    let mut overrides_map = BTreeMap::new();

    for line in overrides.lines() {
        let line = line.expect("No valid line found");
        let mut s = line.split(';');
        let path = s.next().expect("No path in line");
        let toolchain = s.next().expect("No toolchain in line");

        overrides_map.insert(path.into(), toolchain.into());
    }

    let database = OverridesDatabase::Plain(overrides_map);
    toolchain(database);
}

fn settings_toml(mut settings: File) -> Result<(), ()> {
    let mut content = String::new();
    settings.read_to_string(&mut content).expect("Can't read settings file");

    let database = content.parse::<Value>().map_err(|_| ())?;
    let database = database.get("overrides").cloned()
            .and_then(|overrides| overrides.as_table().cloned())
            .and_then(|database| Some(OverridesDatabase::Toml(database)))
            .ok_or(())?;

    toolchain(database);
    Ok(())
}

fn toolchain(database: OverridesDatabase) {
    let mut cwd = match env::current_dir() {
        Ok(cwd) => cwd,
        Err(_) => return,
    };

    loop {
        let path = format!("{}", cwd.display());

        if let Some(toolchain) =  database.get(&path) {
            println!("{}", clean_toolchain_name(toolchain));
            return;
        }

        if !cwd.pop() {
            break;
        }
    }
    println!("default");
}

fn main() {
    let home = env::home_dir().expect("Impossible to get your home dir!");

    let mut overrides_path = home.clone();
    overrides_path.push(OVERRIDES_PATH);

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

    let mut settings_path = home.clone();
    settings_path.push(SETTINGS_PATH);

    if let Ok(f) = File::open(&settings_path) {
        settings_toml(f).unwrap_or_else(|_| println!("default"));
        process::exit(0);
    }

    let mut settings_path = home.clone();
    settings_path.push(OLD_SETTINGS_PATH);

    if let Ok(f) = File::open(&settings_path) {
        settings_toml(f).unwrap_or_else(|_| println!("default"));
        process::exit(0);
    }

    println!("default");
}

#[cfg(test)]
mod test {
    use super::clean_toolchain_name;

    #[test]
    fn simple_name() {
        assert_eq!("nightly", clean_toolchain_name("nightly-x86_64-unknown-linux-gnu"));
        assert_eq!("nightly", clean_toolchain_name("nightly"));
    }

    #[test]
    fn name_with_date() {
        assert_eq!("nightly-2016-06-05", clean_toolchain_name("nightly-2016-06-05-x86_64-unknown-linux-gnu"));
    }
}
