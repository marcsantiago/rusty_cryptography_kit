fn encode(message: &str, key: u8) -> String {
    let key = key % 26;
    message.chars().
        map(|ch|
            if ch.is_alphabetic() {
                let mut value = ch as u8 + key;
                if ch.is_ascii_uppercase() && value < 'A' as u8 {
                    value += 26;
                }
                if ch.is_ascii_uppercase() && value > 'Z' as u8 {
                    value -= 26;
                }
                if ch.is_ascii_lowercase() && value < 'a' as u8 {
                    value += 26;
                }
                if ch.is_ascii_lowercase() && value > 'z' as u8 {
                    value -= 26;
                }
                value as char
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
        assert_eq!("bbbb", &encode("aaaa", 1));
        assert_eq!("zzzz", &encode("aaaa", 25));
        assert_eq!("aaaa", &encode("aaaa", 26));
        assert_eq!("bbbb", &encode("aaaa", 27));
        assert_eq!("gggg26QQGHpgny", &encode("aaaa26KKABjahs", 6));
    }
}


