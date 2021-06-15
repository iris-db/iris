pub trait Decoder<T> {
    fn decode(&self, data: Vec<u8>) -> T;
}
