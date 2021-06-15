/// A JSON document, stored internally using BSON encoding.
pub struct Document {
    data: Vec<u8>,
}

impl Document {
    pub fn new(data: Vec<u8>) -> Document {
        Document { data }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }
}
