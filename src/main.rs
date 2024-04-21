use std::{env, fs};
use rnix::{Parse, Root, SyntaxNode};

// Source taken liberally from https://github.com/nix-community/rnix-parser/blob/master/examples/dump-ast.rs
fn main() {
    let mut iter = env::args().skip(1).peekable();
    if iter.peek().is_none() {
        eprintln!("Usage: dump-ast <file>");
        return;
    }
    for file in iter {
        let content = match fs::read_to_string(file) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("error reading file: {}", err);
                return;
            }
        };
        let parse: Parse<Root> = rnix::Root::parse(&content);

        for error in parse.errors() {
            println!("error: {}", error);
        }

        let syntax_node: SyntaxNode = parse.syntax();
        let json_str = serde_json::to_string(&syntax_node).unwrap();
        println!("{}", json_str);
    }
}
