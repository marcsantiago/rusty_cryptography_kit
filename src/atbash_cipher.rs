use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::Lazy;

type English = Arc<HashMap<char, char>>;

trait SimpleHash<K, V> {
    fn get<Q>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
            Q: std::hash::Hash + Eq + ?Sized;
}

static ENGLISH_ALPHA_HASH: Lazy<English> = Lazy::new(|| {
    let alpha = ('a'..='z').
        filter(|ch| ch.is_ascii_alphabetic());
    let rev = alpha.clone().rev();
    Arc::new(HashMap::from_iter(
        alpha.zip(rev)
    ))
});

static ENGLISH_ALPHA_HASH_REV: Lazy<English> = Lazy::new(|| {
    let alpha = ('a'..='z').
        filter(|ch| ch.is_ascii_alphabetic());
    let rev = alpha.clone().rev();
    Arc::new(HashMap::from_iter(
        rev.zip(alpha)
    ))
});

pub fn encode(message: &str) -> String {
    handle(message, &ENGLISH_ALPHA_HASH)
}

pub fn decode(message: &str) -> String {
    handle(message, &ENGLISH_ALPHA_HASH_REV)
}

fn handle(message: &str, hash: &Lazy<English>) -> String {
    message.chars().
        map(|ch| {
            if ch.is_alphabetic() {
                if ch.is_ascii_uppercase() {
                    let translation = hash.get(&ch.to_ascii_lowercase()).expect("all english letters should be present");
                    return translation.to_ascii_uppercase();
                }
                *hash.get(&ch).expect("all english letters should be present")
            } else {
                ch
            }
        }).collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!("Hzoob hvooh hvz hsvooh yb gsv hvzhsliv.", encode("Sally sells sea shells by the seashore."));
        handle("hello", &ENGLISH_ALPHA_HASH);
    }

    #[test]
    fn test_decode() {
        assert_eq!("Sally sells sea shells by the seashore.", decode("Hzoob hvooh hvz hsvooh yb gsv hvzhsliv."))
    }
}
