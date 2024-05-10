pub mod kinds;

use rnix::{NixLanguage, Parse};
use std::process::exit;
use std::{env, fs};

use crate::kinds::KindType;
use rowan::{GreenNodeBuilder, Language, SyntaxNode};
use serde_json::Value;

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
                        let json_str = to_json(&content);
                        println!("{}", json_str);
                    } else if cmd.as_str() == JSON2NIX {
                        let nix_str = to_nix(&content);
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

pub fn to_nix(path: &String) -> String {
    let json: Value = serde_json::from_str(path.as_str()).unwrap();
    let mut builder = GreenNodeBuilder::new();
    rec(&json, &mut builder);
    let green_node = builder.finish();
    let syntax_node: SyntaxNode<NixLanguage> = SyntaxNode::new_root(green_node);
    format!("{}", syntax_node.text())
}

fn rec(json: &Value, builder: &mut GreenNodeBuilder) {
    let kind_str = json
        .get("kind")
        .expect("Missing 'kind'")
        .as_str()
        .expect("Expected type string for 'kind'");
    let kind_rnix = kinds::from_str(kind_str);
    let kind = NixLanguage::kind_to_raw(kind_rnix);

    match kinds::kind_type(NixLanguage::kind_from_raw(kind)) {
        KindType::TOKEN => builder.token(
            kind,
            json["text"]
                .as_str()
                .expect("Expected type string for 'kind'"),
        ),
        KindType::NODE => builder.start_node(kind),
        KindType::LAST => panic!("developer error"),
    }

    if let Some(children) = json.get("children") {
        if let Some(children) = children.as_array() {
            for child in children {
                rec(child, builder);
            }
        }
    }

    if kinds::kind_type(NixLanguage::kind_from_raw(kind)) == KindType::NODE {
        builder.finish_node();
    }
}

pub fn to_json(content: &String) -> String {
    let parse: Parse<rnix::Root> = rnix::Root::parse(&content);
    for error in parse.errors() {
        eprintln!("error: {}", error);
    }
    let syntax_node: rnix::SyntaxNode = parse.syntax();
    serde_json::to_string_pretty(&syntax_node).unwrap()
}

#[cfg(test)]
mod test {
    use crate::{to_json, to_nix};
    use std::fs;

    #[test]
    fn t1() {
        let nix_str = fs::read_to_string("./example.nix").unwrap();
        let json_str = to_json(&nix_str);
        assert_eq!(nix_str, to_nix(&json_str));
    }

    #[test]
    fn t2() {
        let json_str = fs::read_to_string("./example.json").unwrap();
        let nix_str = to_nix(&json_str);
        assert_eq!(json_str, to_json(&nix_str));
    }
}
