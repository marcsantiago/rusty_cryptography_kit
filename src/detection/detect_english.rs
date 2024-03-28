use crate::detection::trie::TrieIterator;

pub struct Detector {
    trie: crate::detection::trie::Trie,
    threshold: f64,
}


impl Detector {
    pub fn new(trie_gz_path: &str) -> anyhow::Result<Self> {
        let trie = crate::detection::trie::Trie::from_json_file(trie_gz_path)?;
        Ok(Self { trie, threshold: 0.85 })
    }

    pub fn new_with_fix_db() -> anyhow::Result<Self> {
        let trie = crate::detection::trie::Trie::from_json_file("src/detection/trie_db/trie_data.json.gz")?;
        Ok(Self { trie, threshold: 0.85 })
    }


    pub fn set_threshold(&mut self, threshold: f64) -> anyhow::Result<()> {
        anyhow::ensure!(threshold <= 1.0 && threshold > 0.0, "threshold should be greater than 0 and less than or equal to 1");
        Ok(())
    }


    pub fn is_english(&self, text: &str) -> bool {
        let mut english_matches = 0;
        let mut total_words = 0;
        for word in text.split_whitespace() {
            total_words += 1;
            let word: String = word.chars().filter(|ch|
                ch.is_alphabetic()).collect();
            if self.trie.contains(&word) {
                english_matches += 1;
            }
        }
        (english_matches as f64 / total_words as f64) >= self.threshold
    }


    pub fn iter_dictionary_words(&self) -> TrieIterator {
        self.trie.iter_words()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_english() {
        let detector = Detector::new("src/detection/trie_db/trie_data.json.gz").expect("valid detector");
        assert_eq!(true, detector.is_english("hello world"));

        let sample: &str = "On offering to help the blind man, the man who then stole his car, had not, at that precise moment,
  had any evil intention, quite the contrary, what he did was nothing more than obey those feelings of generosity and altruism which,
  as everyone knows, are the two best traits of human nature and to be found in much more hardened criminals than this one,
  a simple car-thief without any hope of advancing in his profession, exploited by the real owners of this enterprise,
  for it is they who take advantage of the needs of the poor.";
        assert_eq!(true, detector.is_english(sample));

        let spanish_sample: &str = "Al ofrecerse a ayudar al ciego, el hombre que luego le robó el coche, en ese preciso momento, no había
  tenía alguna mala intención, todo lo contrario, lo que hizo no fue más que obedecer a esos sentimientos de generosidad y altruismo que,
  como todo el mundo sabe, son los dos mejores rasgos de la naturaleza humana y se encuentran en criminales mucho más empedernidos que éste,
  un simple ladrón de coches sin esperanzas de progresar en su profesión, explotado por los verdaderos dueños de esta empresa,
  porque son ellos quienes se aprovechan de las necesidades de los pobres.";
        assert_eq!(false, detector.is_english(spanish_sample));
    }
}
