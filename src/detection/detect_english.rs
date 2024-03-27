struct Detector {
    trie: crate::detection::trie::Trie,
    threshold: f64,
}


impl Detector {
    pub fn new() -> Self {
        let trie = crate::detection::trie::Trie::from_json_file("src/detection/trie_db/trie_data.json.gz").unwrap();
        Self { trie, threshold: 0.85 }
    }


    pub fn set_threshold(&mut self, threshold: f64) -> anyhow::Result<()> {
        anyhow::ensure!(threshold <= 1.0 && threshold > 0.0, "threshold should be greater than 0 and less than or equal to 1");
    }


    pub fn is_english(&self, text: &str) -> bool {
        let mut english_matches = 0;
        let mut total_words = 0;
        for word in text.split_whitespace() {
            total_words += 1;
            let word = word.chars().filter(|ch|
                !ch.is_alphabetic()).collect();

            if self.trie.contains(word) {
                english_matches += 1;
            }
        }
        (total_words as f64 / english_matches as f64) >= self.threshold
    }
}