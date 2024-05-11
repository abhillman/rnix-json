use std::path::Path;
use std::process::exit;
use std::{env, fs};

use crate::filetype::Filetype;
use crate::filetype::Filetype::{Json, Nix};
use rnix_json::{to_json, to_nix};

pub mod ast_entry;
mod filetype;
pub mod kinds;

fn main() {
    if let Some(arg0) = env::args().nth(1) {
        match Filetype::try_from(Path::new(&arg0)) {
            Ok(filetype) => match fs::read_to_string(&arg0) {
                Ok(content) => {
                    let result = match filetype {
                        Nix => to_json(&content),
                        Json => to_nix(&content),
                    };
                    println!("{}", result);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    exit(1)
                }
            },
            Err(msg) => {
                eprintln!("{}", msg);
                exit(1)
            }
        }
    } else {
        usage_exit();
    }
}

fn usage_exit() {
    eprintln!("Usage: rnix-json <file.(nix|json)>");
    exit(1)
}
