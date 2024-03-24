use base64::{alphabet, engine, Engine};
use base64::engine::GeneralPurpose;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

struct KeyGenerator {
    generated_key: String,
}

impl KeyGenerator {
    fn new() -> Self {
        KeyGenerator {
            generated_key: String::new()
        }
    }
}

impl Iterator for KeyGenerator {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let value: char = if cfg!(test) {
            StdRng::seed_from_u64(self.generated_key.len() as u64).gen_range('A'..='z')
        } else {
            rand::thread_rng().gen_range('A'..='z')
        };
        self.generated_key.push(value);
        Some(value as u8)
    }
}

pub struct PadResponse {
    pub key: String,
    pub encoded_message: String,
}


fn encode(message: &str) -> PadResponse {
    let mut key = KeyGenerator::new();
    let mut encoded_message = String::new();
    for ch in message.chars() {
        let msg_ch = ch as u8;
        let next_k = key.next().expect("excepted a randomly generated character");
        let encoded_value = msg_ch ^ next_k;
        encoded_message.push(encoded_value as char);
    }

    let config = engine::GeneralPurposeConfig::new()
        .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);
    let engine = GeneralPurpose::new(&alphabet::URL_SAFE, config);

    PadResponse {
        key: key.generated_key,
        encoded_message: engine.encode(encoded_message),
    }
}


fn decode(message: &str, key: &str) -> anyhow::Result<String> {
    let config = engine::GeneralPurposeConfig::new()
        .with_decode_padding_mode(engine::DecodePaddingMode::Indifferent);
    let engine = GeneralPurpose::new(&alphabet::URL_SAFE, config);
    let encoded_message = engine.decode(message)?;

    Ok(encoded_message.
        iter().
        zip(key.chars()).map(|(msg_ch, next_k)|
        (msg_ch ^ (next_k as u8)) as char
    ).collect())
}


#[cfg(test)]
mod test {
    use super::*;

    const PLAIN_TEXT: &str = "On offering to help the blind man, the man who then stole his car, had not, at that precise moment,
  had any evil intention, quite the contrary, what he did was nothing more than obey those feelings of generosity and altruism which,
  as everyone knows, are the two best traits of human nature and to be found in much more hardened criminals than this one,
  a simple car-thief without any hope of advancing in his profession, exploited by the real owners of this enterprise,
  for it is they who take advantage of the needs of the poor.";
    const EXPECTED_ENCODED: &str = "IB5lCQ8tIisMDTBuLQ1RGBMMIk4pGRJpBwk5Fj5aOy07RUkQPyR1DhkhaAEzB1ADMiQMWT4TNyUdSiEiKHkVABhaTQE3NGY2CwRIeSwhZSA8FTVtLiUdPzcvAG8wMycJGC9IcExQJzEhaBQjFngoLRwJUDUCJi8XJhAdFEl-Ej4dIAFXFDY_fC8JDx8eNTUYX1gcDSQWVQErezYnLX4eARFAFx86Hz42EUoIHBc4WAQPFAhANjsRHU0iLBckPngACwkBOg8MNVkfK2UVFiQ8ERUKCgQ0dgcIPXAlHwwfFyU2GmYxEgE1K2pXR3YQCn8rPwoEKQobCGUxKj8BC0Z9Kxw5bjY8EFIOGDFxOi0hPmofAhYsIBJXOBdPJjgHIyFZBykGLCo3TigHDmosJnkqE30cOhwlL3o0MncsAz8xeQUzEyR1EioCNC0WIzVuOzEAFTU2CB8YeTYzFBh9Nw82GnMFGA5qS3F5LnYHCzMpHyNyPiwkXQY2ODAvSDk1PBo0OSdOPgAoWiMhOw5lCylAOT0TNw4-Ow8iYyE1bxAtJmcaKxg8NBgxEQwcf0YkPxMiACYkJjtJBANNDgYzQCEWJC9CFSUOLzwDbiopeBUEGCJVFTwaCiIvByg4Jm1lcEEWMjF1IBNEBDRUJQMAA38eIwFQNTsqNWMqMycXHyc2IABzLShSFyURdTAvIgAtRhclZgQdKHwBFxY_Wg==";
    const DECODE_KEY: &str = r#"opEfiKGYecWNYbqpv`Rn]qwIeePxZzVLUiidWAUcxOHv[hpwZAbyMgXIxjIK[YvajvmiVPFXdpdYMUETTtAM^Wx\^\eO]\Jlv[dzlpOPEHuMoXM[uep\lRJyRyrze^cKtTdw`^Z\LfaklTGasxkeEbuiN[RNI^i`b`ypNwWXvjese]xpguf`YYtdmVDxW[XfnlmSakFypMErsJYczycpMVffYPDsxmbLEwFFzhVCF]gVqy_NIovPeumEZDPvxj]Jn\NBTurzo^QXHRJJkpwETawWqoNMjBOyiHrYXRnIijJXIYHv]zUiKKZ]\WAv\YYh\aAUzKpPHxFQNXCix\XiskYB[uv]Cg_iSjvkFAQYOVtb^YsFR]MVpr^QUIhN\Hr[LSn_nQzKNKkEdO`XYeV`]RaECH[OxDUGjYwZQkBxcrSfAGcNoOPC_ifzmznV`SsECbzR`JNpNEOXalqQupRnoP_uAKCAoPap]CUIgdmGtQkez_iKnpAZAPCKWQvqSWGeSBNrcMtU^JGd^fxCFpuM\qxyMt"#;


    #[test]
    fn test_encode() {
        let pad = encode(PLAIN_TEXT);
        assert_eq!(EXPECTED_ENCODED, pad.encoded_message);
    }

    #[test]
    fn test_decode() {
        let message = decode(EXPECTED_ENCODED, DECODE_KEY).expect("no errors");
        assert_eq!(PLAIN_TEXT, message)
    }
}


