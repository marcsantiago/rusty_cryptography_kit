pub fn encode(message: &str, key: u8) -> String {
    let key = key % 26;
    handle(message, key, true)
}


pub fn decode(message: &str, key: u8) -> String {
    let key = key % 26;
    handle(message, key, false)
}

fn handle(message: &str, key: u8, encode: bool) -> String {
    message.chars().
        map(|ch|
            if ch.is_alphabetic() {
                let base = if ch.is_ascii_uppercase() { 'A' as u8 } else { 'a' as u8 };
                let ch = ch as u8;
                let key = if encode { key } else { 26 - key };
                (base + (ch - base + key) % 26) as char
            } else {
                ch
            }
        ).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        // Encode
        assert_eq!("aaaa", &encode("aaaa", 0));
        assert_eq!("bbbb", &encode("aaaa", 1));
        assert_eq!("zzzz", &encode("aaaa", 25));
        assert_eq!("aaaa", &encode("aaaa", 26));
        assert_eq!("bbbb", &encode("aaaa", 27));
        assert_eq!("gggg26QQGHpgnyfd", &encode("aaaa26KKABjahszx", 6));
    }

    #[test]
    fn test_decode() {
        // Decode
        assert_eq!("aaaa", &decode("bbbb", 1));
        assert_eq!("aaaa", &decode("zzzz", 25));
        assert_eq!("aaaa", &decode("aaaa", 26));
        assert_eq!("aaaa", &decode("bbbb", 27));
        assert_eq!("aaaa26KKABjahszx", &decode("gggg26QQGHpgnyfd", 6));
    }
}


