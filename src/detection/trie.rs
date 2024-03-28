use std::collections::HashMap;
use std::io::{BufReader, Read, Write};

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Trie {
    root: TrieNode,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
struct TrieNode {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default)]
    children: HashMap<char, TrieNode>,
    is_end_of_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie { root: TrieNode::default() }
    }

    pub fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.to_ascii_lowercase().chars() {
            current = current.children.entry(ch).or_insert(TrieNode::default());
        }
        current.is_end_of_word = true;
    }

    pub fn contains(&self, word: &str) -> bool {
        let mut current = &self.root;
        for ch in word.to_ascii_lowercase().chars() {
            if let Some(node) = current.children.get(&ch) {
                current = node;
            } else {
                return false;
            }
        }
        current.is_end_of_word
    }

    fn display_words(&self) {
        let mut word = String::new();
        self.display_words_recursive(&self.root, &mut word);
    }

    fn display_words_recursive(&self, node: &TrieNode, prefix: &mut String) {
        for (ch, child) in &node.children {
            prefix.push(*ch);
            if child.is_end_of_word {
                println!("{}", prefix);
            }
            self.display_words_recursive(child, prefix);
            prefix.pop();
        }
    }

    pub fn iter_words(&self) -> TrieIterator {
        TrieIterator::new(&self.root)
    }


    pub fn to_json(&self) -> anyhow::Result<String> {
        serde_json::to_string(self)?
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse json: {}", e))
    }

    pub fn from_json_file(file_path: &str) -> anyhow::Result<Self> {
        let f = std::fs::File::open(file_path)?;
        let reader = BufReader::new(f);
        let mut decoder = GzDecoder::new(reader);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        Ok(serde_json::from_slice(&buffer)?)
    }

    pub fn to_json_file(&self, file_path: &str) -> anyhow::Result<()> {
        let json = self.to_json()?;
        let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(json.as_bytes())?;
        let buffer = encoder.finish()?;
        std::fs::write(file_path, buffer)?;
        Ok(())
    }
}

pub struct TrieIterator {
    stack: Vec<(String, TrieNode)>,
}

impl TrieIterator {
    fn new(node: &TrieNode) -> Self {
        let mut stack = Vec::new();
        stack.push((String::new(), node.clone()));
        TrieIterator { stack }
    }
}

impl Iterator for TrieIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((mut prefix, mut node)) = self.stack.pop() {
            for (ch, child) in node.children.iter_mut() {
                let mut new_prefix = prefix.clone();
                new_prefix.push(*ch);
                self.stack.push((new_prefix, child.clone()));
            }

            if node.is_end_of_word {
                let word = prefix.clone();
                prefix.pop();
                return Some(word);
            }
        }
        None
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

    #[test]
    fn test_display() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("hi");
        trie.insert("hey");
        trie.insert("world");

        let mut words_got: Vec<String> = Vec::new();
        for word in trie.iter_words() {
            words_got.push(word);
        }
        words_got.sort();
        assert_eq!(vec![
            "hello".to_string(), "hey".to_string(),
            "hi".to_string(), "world".to_string()], words_got);
    }
}
