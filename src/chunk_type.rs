mod chunk_type {
    use std::convert::TryFrom;
    use std::str::FromStr;
    use std::str;
    use crate::Error;
    use std::fmt;

    #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
    pub struct ChunkType {
        data: [u8; 4],
    }

    impl ChunkType {
        pub fn is_critical(&self) -> bool {
            self.data[0].is_ascii_uppercase()
        }
        pub fn is_public(&self) -> bool {
            self.data[1].is_ascii_uppercase()
        }
        pub fn is_reserved_bit_valid(&self) -> bool {
            self.data[2].is_ascii_uppercase()
        }
        pub fn is_valid(&self) -> bool {
            self.is_reserved_bit_valid()
        }
        pub fn is_safe_to_copy(&self) -> bool {
            self.data[3].is_ascii_lowercase()
        }
        pub fn bytes(&self) -> [u8; 4] {
            self.data
        }
    }

    impl TryFrom<[u8; 4]> for ChunkType {
        type Error = Error;
        fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
            Ok(Self { data: bytes })
        }
    }

    impl FromStr for ChunkType {
        type Err = Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let bytes = s.as_bytes();
            if bytes.len() != 4 {
               return Err(Box::new(ChunkTypeError::ByteLengthError(bytes.len())));
            }

            if !is_valid_chunk_ascii(bytes) {
                return Err(Box::new(ChunkTypeError::InvalidCharacter));
            }
                let sized: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
                Ok(ChunkType::try_from(sized)?)
        }
    }

    impl fmt::Display for ChunkType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let string = str::from_utf8(&self.data)
                .unwrap()
                .to_string();
            write!(f, "{}", string)
        }
    }
    #[derive(Debug)]
    pub enum ChunkTypeError {
        /// Chunk has incorrect number of bytes (4 expected)
        ByteLengthError(usize),

        /// The input string contains an invalid character at the given index
        InvalidCharacter,
    }
    impl std::error::Error for ChunkTypeError {}

    impl fmt::Display for ChunkTypeError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ChunkTypeError::ByteLengthError(actual) => write!(
                    f,
                    "Expected 4 bytes but received {} when creating chunk type",
                    actual
                ),
                ChunkTypeError::InvalidCharacter => {
                    write!(f, "Input contains one or more invalid characters")
                }
            }
        }
    }

    fn is_valid_chunk_ascii(x: &[u8]) -> bool {
            x.iter().all(|&b| (b >= b'a' && b <= b'z' || (b >= b'A' && b <= b'Z')))
    }

    #[allow(unused_variables)]
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::convert::TryFrom;
        use std::str::FromStr;

        #[test]
        pub fn test_chunk_type_from_bytes() {
            let expected = [82, 117, 83, 116];
            let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

            assert_eq!(expected, actual.bytes());
        }

        #[test]
        pub fn test_chunk_type_from_str() {
            let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
            let actual = ChunkType::from_str("RuSt").unwrap();
            assert_eq!(expected, actual);
        }

        #[test]
        pub fn test_chunk_type_is_critical() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(chunk.is_critical());
        }

        #[test]
        pub fn test_chunk_type_is_not_critical() {
            let chunk = ChunkType::from_str("ruSt").unwrap();
            assert!(!chunk.is_critical());
        }


        #[test]
        pub fn test_chunk_type_is_public() {
            let chunk = ChunkType::from_str("RUSt").unwrap();
            assert!(chunk.is_public());
        }

        #[test]
        pub fn test_chunk_type_is_not_public() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(!chunk.is_public());
        }

        #[test]
        pub fn test_chunk_type_is_reserved_bit_valid() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(chunk.is_reserved_bit_valid());
        }

        #[test]
        pub fn test_chunk_type_is_reserved_bit_invalid() {
            let chunk = ChunkType::from_str("Rust").unwrap();
            assert!(!chunk.is_reserved_bit_valid());
        }

        #[test]
        pub fn test_chunk_type_is_safe_to_copy() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(chunk.is_safe_to_copy());
        }

        #[test]
        pub fn test_chunk_type_is_unsafe_to_copy() {
            let chunk = ChunkType::from_str("RuST").unwrap();
            assert!(!chunk.is_safe_to_copy());
        }

        #[test]
        pub fn test_valid_chunk_is_valid() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert!(chunk.is_valid());
        }

        #[test]
        pub fn test_invalid_chunk_is_valid() {
            let chunk = ChunkType::from_str("Rust").unwrap();
            assert!(!chunk.is_valid());

            let chunk = ChunkType::from_str("Ru1t");
            assert!(chunk.is_err());
        }

        #[test]
        pub fn test_chunk_type_string() {
            let chunk = ChunkType::from_str("RuSt").unwrap();
            assert_eq!(&chunk.to_string(), "RuSt");
        }

        #[test]
        pub fn test_chunk_type_trait_impls() {
            let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
            let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
            let _chunk_string = format!("{}", chunk_type_1);
            let _are_chunks_equal = chunk_type_1 == chunk_type_2;
        }
    }
}
