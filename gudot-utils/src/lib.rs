use gmorph::Enc;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

type Result<T> = std::result::Result<T, String>;

pub fn deserialize_from_file<T: DeserializeOwned, P: AsRef<Path>>(filename: P) -> Result<T> {
    let mut file = File::open(filename.as_ref()).map_err(|err| {
        format!(
            "Failed to open file {}: {}",
            filename.as_ref().display(),
            err
        )
    })?;
    let mut serialized = String::new();
    file.read_to_string(&mut serialized).map_err(|err| {
        format!(
            "Failed to read {} to String: {}",
            filename.as_ref().display(),
            err
        )
    })?;
    serde_json::from_str(&serialized)
        .map_err(|err| format!("Invalid JSON in {}: {}", filename.as_ref().display(), err))
}

pub fn serialize_to_file<T: Serialize, P: AsRef<Path>>(data: T, filename: P) -> Result<()> {
    let mut file = File::create(filename.as_ref()).map_err(|err| {
        format!(
            "Failed to create file {}: {}",
            filename.as_ref().display(),
            err
        )
    })?;
    let serialized = serde_json::to_string(&data)
        .map_err(|err| format!("Failed to convert data to JSON: {}", err))?;
    file.write_all(serialized.as_bytes()).map_err(|err| {
        format!(
            "Failed to write JSON to file {}: {}",
            filename.as_ref().display(),
            err
        )
    })
}

/// Chop as vector into `count` chunks, returning an iterator
pub fn chop<'a>(v: &'a Vec<Enc>, count: usize) -> impl Iterator<Item = Vec<Enc>> + 'a {
    v.chunks(v.len() / count).map(|c| c.to_vec())
}

/// Apply a function to both components of a pair
pub fn both<'a, A, B>(f: impl Fn(&'a A) -> B, pair: &'a (A, A)) -> (B, B) {
    (f(&pair.0), f(&pair.1))
}

/// Zip a pair of iterators, yielding a iterator of pairs
/// zip_pair :: ([a], [b]) -> [(a,b)]
pub fn zip_pair<'a, A, B>(
    pair: (impl Iterator<Item = A> + 'a, impl Iterator<Item = B> + 'a),
) -> impl Iterator<Item = (A, B)> + 'a {
    pair.0.zip(pair.1)
}