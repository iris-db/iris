use crate::io::compressor::compressors::{Snappy, ZLib};

/// An algorithm for compressing a vector of bytes.
pub enum CompressionStrategy {
    Snappy,
    ZLib,
}

impl CompressionStrategy {
    fn compress(&self, buf: &Vec<u8>) -> Vec<u8> {
        let compressor: &dyn Compressor = match self {
            CompressionStrategy::Snappy => &Snappy,
            CompressionStrategy::ZLib => &ZLib,
        };

        let out = compressor.compress(buf);

        out
    }
}

trait Compressor {
    fn compress(&self, input: &Vec<u8>) -> Vec<u8>;
    fn decompress(&self, input: &Vec<u8>) -> Vec<u8>;
}

mod compressors {
    use std::io;
    use std::io::{Cursor, Read, Write};

    use flate2::read::ZlibDecoder;
    use flate2::write::ZlibEncoder;
    use flate2::Compression;

    use super::Compressor;

    pub(crate) struct Snappy;

    impl Compressor for Snappy {
        fn compress(&self, input: &Vec<u8>) -> Vec<u8> {
            let mut wtr = snap::write::FrameEncoder::new(Vec::new());

            wtr.write_all(input).unwrap();
            wtr.into_inner().unwrap()
        }

        fn decompress(&self, input: &Vec<u8>) -> Vec<u8> {
            let mut buf = Vec::new();
            snap::read::FrameDecoder::new(input.as_slice()).read_to_end(&mut buf);

            buf
        }
    }

    pub(crate) struct ZLib;

    impl Compressor for ZLib {
        fn compress(&self, input: &Vec<u8>) -> Vec<u8> {
            let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
            e.write_all(input);
            e.finish().unwrap()
        }

        fn decompress(&self, input: &Vec<u8>) -> Vec<u8> {
            let mut e = ZlibDecoder::new(Cursor::new(input));

            let mut out = Vec::new();
            e.read_to_end(&mut out);

            out
        }
    }

    #[cfg(test)]
    mod tests {
        use std::fs;

        use serde_json::json;

        use crate::lib::json::bsonio::encoder::encode_json_object;
        use crate::lib::json::types::JsonObject;

        use super::*;

        #[test]
        fn test_snappy() {
            let json_value = json!({
                "username": "Stevemmmmm",
                "password": "g$ae89ru8q39ilozxcji8oqu3ruiopfhj7i8oqu3rhioudhf",
                "address": {
                    "street": "128901234u89 Object",
                    "city": "ABC",
                    "state": "GE",
                    "country": "SOM"
                },
                "friends": [
                    "UUID(ABC)",
                    "UUID(ABC)",
                    "UUID(ABC)",
                ]
            });

            let bytes = encode_json_object(json_value.as_object().unwrap().clone());

            let out = Snappy.compress(&bytes);

            assert_eq!(bytes, Snappy.decompress(&out));
        }

        #[test]
        fn test_zlib() {
            let json_value = json!({
                "username": "Stevemmmmm",
                "password": "g$ae89ru8q39ilozxcji8oqu3ruiopfhj7i8oqu3rhioudhf",
                "address": {
                    "street": "128901234u89 Object",
                    "city": "ABC",
                    "state": "GE",
                    "country": "SOM"
                },
                "friends": [
                    "UUID(ABC)",
                    "UUID(ABC)",
                    "UUID(ABC)",
                ]
            });

            let bytes = encode_json_object(json_value.as_object().unwrap().clone());

            let out = ZLib.compress(&bytes);

            assert_eq!(bytes, ZLib.decompress(&out));
        }
    }
}
