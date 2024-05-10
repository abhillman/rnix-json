use rnix::NixLanguage;
use rowan::Language;
use serde::{Deserialize, Serialize};

use crate::kinds;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum AstEntry {
    Node {
        kind: String,
        text_range: Vec<u32>,
        children: Vec<AstEntry>,
    },
    Token {
        kind: String,
        text_range: Vec<u32>,
        text: String,
    },
}

impl AstEntry {
    fn kind_str(&self) -> &str {
        match self {
            AstEntry::Node { kind, .. } => kind,
            AstEntry::Token { kind, .. } => kind,
        }
    }

    pub fn rnix_kind(&self) -> rnix::SyntaxKind {
        kinds::from_str(self.kind_str())
    }

    pub fn raw_kind(&self) -> rowan::SyntaxKind {
        NixLanguage::kind_to_raw(self.rnix_kind())
    }
}

#[cfg(test)]
mod test {
    use crate::ast_entry::AstEntry;

    #[test]
    fn t1() {
        let data = std::fs::read_to_string("./example.json").unwrap();
        let ast_entry: AstEntry = serde_json::from_str(data.as_str()).unwrap();

        println!("{:#?}", ast_entry);
    }
}
