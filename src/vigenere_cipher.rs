pub fn encode(message: &str, key: &str) -> String {
    handle(message, key, true)
}


pub fn decode(message: &str, key: &str) -> String {
    handle(message, key, false)
}

fn handle(message: &str, key: &str, encode: bool) -> String {
    let mut key = key.chars().
        filter(|x| !x.is_whitespace() || !x.is_ascii_alphanumeric()).
        cycle();

    message.chars()
        .map(|ch| {
            if ch.is_alphabetic() {
                let k = key.next().expect("next letter in key failed");
                let k = k.to_ascii_lowercase() as u8 - 'a' as u8;
                let k = if encode { k } else { 26 - k };
                let base = if ch.is_ascii_uppercase() { 'A' as u8 } else { 'a' as u8 };
                let ch = ch as u8;
                (base + (ch - base + k) % 26) as char
            } else {
                ch
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        // Encode
        assert_eq!("Se arrxts, oeu tuh cchdr nzly dcnxwh pfu. Oh dpkhbxvd, nqd ealm nzly pavx fcl khrlr wxhrvi -Wuhew Hm Hzde", encode("Be normal, and the crowd will accept you. Be deranged, and they will make you their leader -Wheel Of Time", "randalthor"));
        
        // Decode
        assert_eq!("Be normal, and the crowd will accept you. Be deranged, and they will make you their leader -Wheel Of Time", decode("Se arrxts, oeu tuh cchdr nzly dcnxwh pfu. Oh dpkhbxvd, nqd ealm nzly pavx fcl khrlr wxhrvi -Wuhew Hm Hzde", "randalthor"));
    }
}
