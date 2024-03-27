use clap::Parser;
use clap_derive::Parser;

use rusty_cryptography_kit::detection::trie::Trie;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, required = true)]
    world_list_path: String,
    #[arg(short, default_value = "src/detection/trie_db/trie_data.json.gz")]
    save_trie_data_path: String,
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let mut trie = Trie::new();
    let words = std::fs::read_to_string(args.world_list_path)?;
    for word in words.lines() {
        trie.insert(word);
    }
    trie.to_json_file(&args.save_trie_data_path)?;
    Ok(())
}