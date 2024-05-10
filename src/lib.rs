use rnix::{NixLanguage, Parse};
use rowan::{GreenNodeBuilder, SyntaxNode};

pub mod ast_entry;
pub mod kinds;

pub fn to_nix(json_content: &str) -> String {
    fn rec(ast_entry: crate::ast_entry::AstEntry, builder: &mut GreenNodeBuilder) {
        let kind = ast_entry.raw_kind();
        match ast_entry {
            crate::ast_entry::AstEntry::Node { children, .. } => {
                builder.start_node(kind);
                for child in children {
                    rec(child, builder)
                }
                builder.finish_node()
            }
            crate::ast_entry::AstEntry::Token { text, .. } => builder.token(kind, text.as_str()),
        }
    }

    let ast_entry: crate::ast_entry::AstEntry = serde_json::from_str(json_content).unwrap();
    let mut builder = GreenNodeBuilder::new();
    rec(ast_entry, &mut builder);

    let green_node = builder.finish();
    let syntax_node: SyntaxNode<NixLanguage> = SyntaxNode::new_root(green_node);
    format!("{}", syntax_node.text())
}

pub fn to_json(nix_content: &str) -> String {
    let parse: Parse<rnix::Root> = rnix::Root::parse(nix_content);
    for error in parse.errors() {
        eprintln!("error: {}", error);
    }
    let syntax_node: rnix::SyntaxNode = parse.syntax();
    serde_json::to_string_pretty(&syntax_node).unwrap()
}

#[cfg(test)]
mod test {
    use std::fs;

    use crate::{to_json, to_nix};

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
