use crate::lib::common::JsonObject;
use bson::Bson;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Cursor, Error, Write};

pub fn write_chunk(path: &str, bson: Vec<u8>) -> Result<(), Error> {
    let mut buf: Vec<u8> = Vec::new();

    buf.append(&mut bson.clone());
    buf.push(b'\n');
    buf.push(b'\n');

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("Affinity/{}", path))?;

    file.write_all(buf.as_ref())?;

    Ok(())
}

pub fn iter2(path: &str) -> Vec<JsonObject> {
    let f = File::open(format!("Affinity/{}", path)).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();

    let mut acc: Vec<JsonObject> = Vec::new();

    loop {
        let bytes = reader.read_until(b'\n', &mut buf).unwrap();
        if bytes == 0 {
            break;
        }

        // BSON block delimiter.
        if bytes == 1 && buf[buf.len() - 1] == b'\n' {
            let doc = bson::Document::from_reader(&mut Cursor::new(&buf[..]));
            let doc = match doc {
                Ok(v) => v,
                Err(_) => break,
            };

            let res: Result<Bson, _> = bson::from_document(doc);
            let json_v = res.unwrap().into_relaxed_extjson();
            let json = json_v.as_object().unwrap();

            acc.push(json.clone());

            buf.clear();
        }
    }

    acc
}
