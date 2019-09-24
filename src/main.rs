use gmorph::{Decrypt, Enc, Encrypt, KeyPair};
use std::{
    fs::File,
    io::{Read, Write},
};
use structopt::StructOpt;

type GuDotResult = std::result::Result<(), String>;

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

fn generate_impl() -> GuDotResult {
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
    let serialized = serde_json::to_string(&(x, y))
        .map_err(|err| format!("Failed to serialize vectors: {}", err))?;
    let mut data_file = File::create(FILENAME)
        .map_err(|err| format!("Failed to create file {}: {}", FILENAME, err))?;
    data_file
        .write_all(serialized.as_bytes())
        .map_err(|err| format!("Failed to write to file {}: {}", FILENAME, err))
}

fn encrypt_impl() -> GuDotResult {
    fn encrypt_vec(key_pair: &KeyPair, v: Vec<u32>) -> Vec<Enc> {
        v.into_iter().map(|x| Enc::encrypt(&key_pair, x)).collect()
    }

    const INPUT_FN: &str = "input.json";
    const KEYS_FN: &str = "keys.json";
    const OUTPUT_FN: &str = "enc_input.json";

    // input.json of the form
    // [[1,2,3],[2,4,6]]
    let key_pair = KeyPair::new();
    let mut vectors_file =
        File::open(INPUT_FN).map_err(|err| format!("Failed to open file {}: {}", INPUT_FN, err))?;

    let mut serialized_input = String::new();
    vectors_file
        .read_to_string(&mut serialized_input)
        .map_err(|err| format!("Failed to read file {}: {}", INPUT_FN, err))?;

    let (x, y): (Vec<u32>, Vec<u32>) = serde_json::from_str(&serialized_input)
        .map_err(|err| format!("Failed to deserialize input vectors: {}", err))?;

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

    let mut data_file = File::create(OUTPUT_FN)
        .map_err(|err| format!("Failed to create file {}: {}", OUTPUT_FN, err))?;
    data_file
        .write_all(data.as_bytes())
        .map_err(|err| format!("Failed to write file {}: {}", OUTPUT_FN, err))?;

    let mut keys_file = File::create(KEYS_FN)
        .map_err(|err| format!("Failed to create file {}: {}", KEYS_FN, err))?;
    keys_file
        .write_all(serialized_keypair.as_bytes())
        .map_err(|err| format!("Failed to write {}: {}", KEYS_FN, err))
}

fn decrypt_impl() -> GuDotResult {
    const KEYS_FN: &str = "keys.json";
    const INPUT_FN: &str = "enc_output.json";
    const OUTPUT_FN: &str = "output.json";

    let mut keys_file =
        File::open(KEYS_FN).map_err(|err| format!("Failed to open file {}: {}", KEYS_FN, err))?;
    let mut serialized_keypair = String::new();
    keys_file
        .read_to_string(&mut serialized_keypair)
        .map_err(|err| format!("Failed to read {} to String: {}", KEYS_FN, err))?;
    let key_pair: KeyPair = serde_json::from_str(&serialized_keypair)
        .map_err(|err| format!("Invalid JSON in {}: {}", KEYS_FN, err))?;

    let mut data_file =
        File::open(INPUT_FN).map_err(|err| format!("Failed to open file {}: {}", INPUT_FN, err))?;
    let mut serialized_data = String::new();
    data_file
        .read_to_string(&mut serialized_data)
        .map_err(|err| format!("Failed to read {} to String: {}", INPUT_FN, err))?;
    let data: Vec<(Enc, Enc)> = serde_json::from_str(&serialized_data)
        .map_err(|err| format!("Invalid JSON in {}: {}", INPUT_FN, err))?;

    // decrypt
    let data: Vec<_> = data
        .into_iter()
        .map(|(a, b)| (a.decrypt(&key_pair), b.decrypt(&key_pair)))
        .collect();

    let mut data_file = File::create(OUTPUT_FN)
        .map_err(|err| format!("Failed to create file {}: {}", OUTPUT_FN, err))?;
    let serialized_data = serde_json::to_string(&data)
        .map_err(|err| format!("Failed to convert decrypted data to JSON: {}", err))?;
    data_file
        .write_all(serialized_data.as_bytes())
        .map_err(|err| format!("Failed to write JSON to file {}: {}", OUTPUT_FN, err))
}

fn regress_impl() -> GuDotResult {
    Ok(())
}

fn plot_impl() -> GuDotResult {
    Ok(())
}
