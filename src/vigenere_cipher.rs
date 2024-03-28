pub fn encode(message: &str, key: &str) -> String {
    handle(message, key, true)
}


pub fn decode(message: &str, key: &str) -> String {
    handle(message, key, false)
}

#[derive(Debug, PartialEq)]
pub struct BruteForceResponse {
    decoded_message: String,
    key: String,
}

pub fn decode_brute_force(message: &str, limit: usize) -> anyhow::Result<BruteForceResponse> {
    let detection = crate::detection::detect_english::Detector::new_with_fix_db()?;
    let iter = detection.iter_dictionary_words().
        filter(|s| {
            !s.is_empty() || !s.is_ascii()
        }).
        take(limit);

    for possible_key in iter {
        if possible_key.len() == 0 {
            continue;
        }
        let plain_text = decode(message, &possible_key);
        if detection.is_english(&plain_text) {
            return Ok(BruteForceResponse { decoded_message: plain_text, key: possible_key });
        }
    }
    anyhow::bail!("message could not be decoded")
}

fn handle(message: &str, key: &str, encode: bool) -> String {
    let mut key = key.chars().
        filter(|x| !x.is_whitespace() || !x.is_ascii_alphanumeric()).
        cycle();

    message.chars()
        .map(|ch| {
            if ch.is_alphabetic() {
                let k = key.next().expect("next letter in key failed");

                if k.to_ascii_lowercase() as usize > u8::MAX as usize {
                    return ch;
                }
                if (k.to_ascii_lowercase() as isize - 'a' as isize) < 0 {
                    return ch;
                }

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
        assert_eq!("Se arrxts, oeu tuh cchdr nzly dcnxwh pfu. Oh dpkhbxvd, nqd ealm nzly pavx fcl khrlr wxhrvi -Wuhew Hm Hzde", encode("Be normal, and the crowd will accept you. Be deranged, and they will make you their leader -Wheel Of Time", "randalthor"));
    }

    #[test]
    fn test_decode() {
        assert_eq!("Be normal, and the crowd will accept you. Be deranged, and they will make you their leader -Wheel Of Time", decode("Se arrxts, oeu tuh cchdr nzly dcnxwh pfu. Oh dpkhbxvd, nqd ealm nzly pavx fcl khrlr wxhrvi -Wuhew Hm Hzde", "randalthor"));
    }


    #[test]
    fn test_decode_brute_force() {
        assert_eq!(BruteForceResponse {
            decoded_message: "Be normal, and the crowd will accept you. Be deranged, and they will make you their leader -Wheel Of Time".to_string(),
            key: "love".to_string(),
        }, decode_brute_force("Ms iscavp, lby xss xvzky atzg enqzte mjy. Ms yicoikpr, vro hcij kdpw avop mjy evzmc zzeosm -Asszp Zt Omxs", usize::MAX).expect("should be able to decode simple key"));
    }
}
