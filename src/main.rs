use gmorph::{Enc, Encrypt, KeyPair};
use std::{
    fs::File,
    io::{Read, Write},
};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum GuDot {
    /// Generate test input data
    #[structopt(name = "generate")]
    Generate,
    /// Encrypt test input data
    #[structopt(name = "encrypt")]
    Encrypt,
}

fn main() {
    let res = match GuDot::from_args() {
        GuDot::Generate => generate_impl(),
        GuDot::Encrypt => encrypt_impl(),
    };
    if let Err(err) = res {
        eprintln!("Error occurred: {}", err);
    }
}

fn generate_impl() -> Result<(), String> {
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
    let serialized =
        serde_json::to_string(&(x, y)).map_err(|_| "Failed to serialize vectors".to_string())?;
    let mut data_file =
        File::create("input.json").map_err(|_| "Failed to create input.json".to_string())?;
    data_file
        .write_all(serialized.as_bytes())
        .map_err(|_| "Failed to write input.json".to_string())
}

fn encrypt_impl() -> Result<(), String> {
    fn encrypt_vec(key_pair: &KeyPair, v: Vec<u32>) -> Vec<Enc> {
        v.into_iter().map(|x| Enc::encrypt(&key_pair, x)).collect()
    }

    // input.json of the form
    // [[1,2,3],[2,4,6]]
    let key_pair = KeyPair::new();
    let mut vectors_file =
        File::open("input.json").map_err(|_| "Failed to open input.json".to_string())?;

    let mut serialized_input = String::new();
    vectors_file
        .read_to_string(&mut serialized_input)
        .map_err(|_| "Failed to read input.json".to_string())?;

    let (x, y): (Vec<u32>, Vec<u32>) = serde_json::from_str(&serialized_input)
        .map_err(|_| "Failed to deserialize input vectors".to_string())?;

    let first_x: u32 = x[0];
    let first_y: u32 = y[0];
    let last_y: u32 = *y.last().unwrap();

    let x1: Vec<u32> = x.into_iter().map(|v| v - first_x).collect();
    let y1: Vec<u32> = if last_y < first_y {
        y.into_iter().map(|v| first_y - v).collect()
    } else {
        y.into_iter().map(|v| v - first_y).collect()
    };
    let enc_x = encrypt_vec(&key_pair, x1);
    let enc_y = encrypt_vec(&key_pair, y1);

    let data = serde_json::to_string(&(enc_x, enc_y))
        .map_err(|err| format!("Couldn't convert encrypted data to JSON: {}", err))?;
    let serialized_keypair = serde_json::to_string(&key_pair)
        .map_err(|err| format!("Couldn't convert key-pair to JSON: {}", err))?;

    let mut data_file =
        File::create("data.json").map_err(|_| "Failed to create data.json".to_string())?;
    data_file
        .write_all(data.as_bytes())
        .map_err(|_| "Failed to write data.json".to_string())?;

    let mut keys_file =
        File::create("keys.json").map_err(|_| "Failed to create keys.json".to_string())?;
    keys_file
        .write_all(serialized_keypair.as_bytes())
        .map_err(|_| "Failed to write keys.json".to_string())
}
