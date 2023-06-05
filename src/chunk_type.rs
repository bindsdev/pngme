#![allow(dead_code)]

use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    str::{self, FromStr},
};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType {
    inner: u32,
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.inner.to_be_bytes()
    }

    fn is_valid(&self) -> bool {
        // There are a few requirements for a valid chunk type:
        // - must be represented by 4 characters (or bytes)
        // - must only contain alphabetic characters
        // - 3rd character must be uppercase
        //
        // The first two are already checked in the `FromStr` implementation. We only need to check
        // if the 3rd character is uppercase here.
        char::from(self.bytes()[2]).is_uppercase()
    }

    fn is_critical(&self) -> bool {
        // If the 5th bit of the 1st byte is not set, the chunk is critical.
        ((self.bytes()[0] >> 5) & 1) == 0
    }

    fn is_public(&self) -> bool {
        // If the 5th bit of the 2nd byte is not set, the chunk is public.
        ((self.bytes()[1] >> 5) & 1) == 0
    }

    fn is_reserved_bit_valid(&self) -> bool {
        // If the 5th bit of the 3rd byte is not set, the chunk is reserved.
        ((self.bytes()[2] >> 5) & 1) == 0
    }

    fn is_safe_to_copy(&self) -> bool {
        // If the 5th bit of the 4th byte is set, the chunk is safe to copy.
        ((self.bytes()[3] >> 5) & 1) == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: u32::from_be_bytes(value),
        })
    }
}

impl FromStr for ChunkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.chars().all(char::is_alphabetic) {
            return Err(());
        }

        Ok(Self {
            inner: u32::from_be_bytes(s.as_bytes().try_into().unwrap()),
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            str::from_utf8(&self.inner.to_be_bytes()).map_err(|_| fmt::Error)?
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{convert::TryFrom, str::FromStr};

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
