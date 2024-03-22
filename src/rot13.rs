pub fn encode(message: &str) -> String {
    crate::ceaser_cipher::encode(message, 13)
}


pub fn decode(message: &str) -> String {
    crate::ceaser_cipher::decode(message, 13)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_encode() {
        // Encode
        assert_eq!("nnnn", &encode("aaaa"));
        assert_eq!("nnnn26XXNOwnufmk", &encode("aaaa26KKABjahszx"));

        // Decode
        assert_eq!("aaaa26KKABjahszx", &decode("nnnn26XXNOwnufmk"));
    }
}


