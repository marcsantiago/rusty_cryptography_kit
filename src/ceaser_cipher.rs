pub fn encode(message: &str, key: u8) -> String {
    let key = key % 26;
    handle(message, key as isize)
}


pub fn decode(message: &str, key: u8) -> String {
    let key = key % 26;
    let key: isize = (key as isize) * -1;
    handle(message, key)
}

fn handle(message: &str, key: isize) -> String {
    message.chars().
        map(|ch|
            if ch.is_alphabetic() {
                let mut value = (ch as isize) + key;
                if ch.is_ascii_uppercase() {
                    if value < 'A' as isize {
                        value += 26;
                    } else if value > 'Z' as isize {
                        value -= 26;
                    }
                } else if ch.is_ascii_lowercase() {
                    if value < 'a' as isize {
                        value += 26;
                    } else if value > 'z' as isize {
                        value -= 26;
                    }
                }
                (value as u8) as char
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
        assert_eq!("bbbb", &encode("aaaa", 1));
        assert_eq!("zzzz", &encode("aaaa", 25));
        assert_eq!("aaaa", &encode("aaaa", 26));
        assert_eq!("bbbb", &encode("aaaa", 27));
        assert_eq!("gggg26QQGHpgnyfd", &encode("aaaa26KKABjahszx", 6));

        // Decode
        assert_eq!("aaaa", &decode("bbbb", 1));
        assert_eq!("aaaa", &decode("zzzz", 25));
        assert_eq!("aaaa", &decode("aaaa", 26));
        assert_eq!("aaaa", &decode("bbbb", 27));
        assert_eq!("aaaa26KKABjahszx", &decode("gggg26QQGHpgnyfd", 6));
    }
}


