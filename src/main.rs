use gmorph::{Decrypt, Enc, Encrypt, KeyPair};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};
use structopt::StructOpt;

type Result<T> = std::result::Result<T, String>;

#[derive(StructOpt, Debug)]
enum GuDot {
    /// Generate test input data
    #[structopt(name = "generate")]
    Generate,
    /// Encrypt input data
    #[structopt(name = "encrypt")]
    Encrypt,
    /// Decrypt output data
    #[structopt(name = "decrypt")]
    Decrypt,
    /// Regress output data
    #[structopt(name = "regress")]
    Regress,
    /// Plot input data with/without regressed line
    #[structopt(name = "plot")]
    Plot,
}

fn main() {
    let res = match GuDot::from_args() {
        GuDot::Generate => generate_impl(),
        GuDot::Encrypt => encrypt_impl(),
        GuDot::Decrypt => decrypt_impl(),
        GuDot::Regress => regress_impl(),
        GuDot::Plot => plot_impl(),
    };
    if let Err(err) = res {
        eprintln!("Error occurred: {}", err);
    }
}

fn generate_impl() -> Result<()> {
    const FILENAME: &str = "input.json";

    //    let x = vec!(1,2,3,4);
    //    let y = vec!(2,4,6,8);
    let v = 2.71;
    let t0 = 9 * 3600;
    let d0 = 6624;

    let mut x = Vec::new();
    let mut y = Vec::new();

    for i in 1..100 {
        let t = t0 + i;
        let dd = (v * (i as f64)).round() as u32;
        let d = d0 - dd;
        x.push(t);
        y.push(d);
    }

    serialize_to_file((x, y), FILENAME)
}

fn encrypt_impl() -> Result<()> {
    fn encrypt_vec(key_pair: &KeyPair, v: Vec<u32>) -> Vec<Enc> {
        v.into_iter().map(|x| Enc::encrypt(&key_pair, x)).collect()
    }

    const INPUT_FN: &str = "input.json";
    const KEYS_FN: &str = "keys.json";
    const OUTPUT_FN: &str = "enc_input.json";

    // input.json of the form
    // [[1,2,3],[2,4,6]]
    let key_pair = KeyPair::new();
    let (x, y): (Vec<u32>, Vec<u32>) = deserialize_from_file(INPUT_FN)?;

    let first_x: u32 = x[0];
    let first_y: u32 = y[0];
    let last_y = y
        .last()
        .ok_or("Expected at least one element in the input vector".to_string())?;

    let x1: Vec<u32> = x.into_iter().map(|v| v - first_x).collect();
    let y1: Vec<u32> = if *last_y < first_y {
        y.into_iter().map(|v| first_y - v).collect()
    } else {
        y.into_iter().map(|v| v - first_y).collect()
    };
    let enc_x = encrypt_vec(&key_pair, x1);
    let enc_y = encrypt_vec(&key_pair, y1);

    serialize_to_file((enc_x, enc_y), OUTPUT_FN)?;
    serialize_to_file(key_pair, KEYS_FN)
}

fn decrypt_impl() -> Result<()> {
    const KEYS_FN: &str = "keys.json";
    const INPUT_FN: &str = "enc_output.json";
    const OUTPUT_FN: &str = "output.json";

    let key_pair: KeyPair = deserialize_from_file(KEYS_FN)?;
    let data: Vec<(Enc, Enc)> = deserialize_from_file(INPUT_FN)?;

    // decrypt
    let data: Vec<_> = data
        .into_iter()
        .map(|(a, b)| (a.decrypt(&key_pair), b.decrypt(&key_pair)))
        .collect();

    serialize_to_file(data, OUTPUT_FN)
}

fn regress_impl() -> Result<()> {
    const INPUT_FN: &str = "output.json";
    const OUTPUT_FN: &str = "regress.json";

    let fitted: Vec<(u32, u32)> = deserialize_from_file(INPUT_FN)?;
    let (a, b) = fitted
        .into_iter()
        .fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b));
    let coeff = a as f64 / b as f64;

    serialize_to_file(coeff, OUTPUT_FN)
}

fn plot_impl() -> Result<()> {
    Ok(())
}

fn deserialize_from_file<T: DeserializeOwned, P: AsRef<Path>>(filename: P) -> Result<T> {
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

fn serialize_to_file<T: Serialize, P: AsRef<Path>>(data: T, filename: P) -> Result<()> {
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
