use std::process::exit;
use std::{env, fs};

pub mod ast_entry;
pub mod kinds;

const JSON2NIX: &str = "json2nix";
const NIX2JSON: &str = "nix2json";

fn main() {
    let mut iter = env::args().skip(1).peekable();
    match iter.next() {
        None => usage_exit(),
        Some(cmd) => match cmd.as_str() {
            JSON2NIX | NIX2JSON => match iter.next() {
                None => usage_exit(),
                Some(path) => {
                    let content = match fs::read_to_string(path) {
                        Ok(content) => content,
                        Err(err) => {
                            eprintln!("error reading file: {}", err);
                            exit(1)
                        }
                    };

                    if cmd.as_str() == NIX2JSON {
                        let json_str = rnix_json::to_json(&content);
                        println!("{}", json_str);
                    } else if cmd.as_str() == JSON2NIX {
                        let nix_str = rnix_json::to_nix(&content);
                        println!("{}", nix_str)
                    } else {
                        usage_exit();
                    }
                }
            },
            &_ => usage_exit(),
        },
    }
}

fn usage_exit() {
    eprintln!("Usage: rnix-json <{}|{}> <file>", JSON2NIX, NIX2JSON);
    exit(1)
}
