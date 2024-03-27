use std::collections::HashMap;
use std::io::Write;

use serde::{Deserialize, Serialize};

// https://dev.to/timclicks/two-trie-implementations-in-rust-ones-super-fast-2f3m
#[derive(Default, Debug, Serialize, Deserialize)]
struct TrieNode {
    is_end_of_word: bool,
    children: HashMap<u8, TrieNode>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current_node = &mut self.root;
        for c in word.to_ascii_lowercase().bytes() {
            current_node = current_node.children.entry(c).or_default();
        }
        current_node.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current_node = &self.root;
        for c in word.to_ascii_lowercase().bytes() {
            match current_node.children.get(&c) {
                Some(node) => current_node = node,
                None => return false,
            }
        }
        current_node.is_end_of_word
    }


    pub fn to_json(&self) -> anyhow::Result<String> {
        serde_json::to_string(self)?
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse json: {}", e))
    }

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(json).unwrap()
    }

    pub fn from_json_file(file_path: &str) -> anyhow::Result<Self> {
        let json = std::fs::read_to_string(file_path)?;
        Ok(Self::from_json(&json))
    }

    pub fn to_json_file(&self, file_path: &str) -> anyhow::Result<()> {
        let json = self.to_json()?;
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(json.as_bytes())?;
        let buffer = encoder.finish()?;
        std::fs::write(file_path, buffer)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trie() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("hi");
        trie.insert("hey");
        trie.insert("world");


        assert_eq!(trie.contains("hello"), true);
        assert_eq!(trie.contains("Hello"), true);
        assert_eq!(trie.contains("hello world"), false);

        trie.insert("hello world");
        assert_eq!(trie.contains("hello world"), true);
    }
}
