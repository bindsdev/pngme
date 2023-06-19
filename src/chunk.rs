use crate::chunk_type::ChunkType;
use std::{
    convert::TryFrom,
    fmt::{self, Display, Formatter},
    io::{BufReader, Cursor, Read, Seek, SeekFrom}
};

#[derive(Debug)]
struct Chunk {
    ctype: ChunkType,
    cdata: Vec<u8>,
}

impl Chunk {
    fn new(ctype: ChunkType, cdata: Vec<u8>) -> Self {
        Self { ctype, cdata }
    }

    fn length(&self) -> u32 {
        todo!();
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.ctype
    }

    fn data(&self) -> &[u8] {
        &self.cdata
    }

    fn crc(&self) -> u32 {
        todo!();
    }

    fn data_as_string(&self) -> crate::Result<String> {
        let data = self.data();

        let mut reader = BufReader::new(&data[4..(data.len() - 4)]);
        let mut data_str_bytes = Vec::new();
        reader.read_to_end(&mut data_str_bytes)?;

        Ok(String::from_utf8(data_str_bytes)?)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = crate::Error;

    fn try_from(value: &[u8]) -> crate::Result<Self> {
        let mut reader = BufReader::new(Cursor::new(value));

        let mut ctype_buf: [u8; 4] = [0, 0, 0, 0];
        reader.seek(SeekFrom::Start(4))?;
        reader.read_exact(&mut ctype_buf)?;
        let ctype = ChunkType::try_from(ctype_buf)?;

        reader.rewind()?;

        let cdata_handle = &mut value[..4].chain(&value[8..]);
        let mut cdata = Vec::new();
        cdata_handle.read_to_end(&mut cdata)?;

        Ok(Self { ctype, cdata })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Length: {}", self.length())?;
        writeln!(f, "Type: {}", self.chunk_type())?;
        writeln!(f, "Data: {} bytes", self.data().len())?;
        writeln!(f, "CRC: {}", self.crc())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
