use std::io;
use std::io::Cursor;

/// An algorithm for compression a vector of bytes.
pub enum CompressionStrategy {
	Snappy(Vec<u8>),
	ZLib(Vec<u8>),
}

impl CompressionStrategy {
	pub fn compress(&self) -> Vec<u8> {
		return match self {
			CompressionStrategy::Snappy(buf) => Snappy.compress(buf),
			CompressionStrategy::ZLib(buf) => ZLib.compress(buf),
		};
	}
}

trait Compressor {
	fn compress(&self, buf: &Vec<u8>) -> Vec<u8>;
}

struct Snappy;

impl Compressor for Snappy {
	fn compress(&self, buf: &Vec<u8>) -> Vec<u8> {
		let mut buf = buf.clone();

		let mut out: Vec<u8> = Vec::new();

		let mut wtr = snap::write::FrameEncoder::new(Cursor::new(&mut out));
		std::io::copy(&mut Cursor::new(buf), &mut wtr);

		Vec::new()
	}
}

struct ZLib;

impl Compressor for ZLib {
	fn compress(&self, buf: &Vec<u8>) -> Vec<u8> {
		let mut out: Vec<u8> = Vec::new();
		out
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_snappy() {
		let input = b"Hello world".to_vec();

		let res = Snappy.compress(&input);

		println!("{:?}\n{:?}", input, res);

		assert!(res.len() < input.len());
	}
}
