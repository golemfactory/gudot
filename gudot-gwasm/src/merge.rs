use super::utils::both;
use gmorph::{Decrypt, Enc, KeyPair};
use std::{fs::File, io::Read, process};

pub(crate) fn merge(_args: &Vec<String>, results: Vec<((Vec<Enc>, Vec<Enc>), (Enc, Enc))>) {
    if let Err(err) = merge_impl(results) {
        eprintln!("Error occurred while merging results: {}", err);
        process::exit(1);
    }
}

fn merge_impl(results: Vec<((Vec<Enc>, Vec<Enc>), (Enc, Enc))>) -> Result<(), String> {
    let mut keys_file =
        File::open("keys.json").map_err(|_| "Failed to open keys.json".to_string())?;
    let mut serialized_keypair = String::new();
    keys_file
        .read_to_string(&mut serialized_keypair)
        .map_err(|_| "Failed to read keys.json to String".to_string())?;
    let key_pair: KeyPair = serde_json::from_str(&serialized_keypair)
        .map_err(|err| format!("Invalid JSON in keys.json: {}", err))?;

    let mut input_file = File::open("input.json").expect("Failed to open input.json");

    let mut serialized_input = String::new();
    input_file
        .read_to_string(&mut serialized_input)
        .expect("Failed to read input.json");

    let (x, y): (Vec<u32>, Vec<u32>) =
        serde_json::from_str(&serialized_input).expect("Failed to deserialize input vectors");

    let first_x: u32 = x[0];
    let first_y: u32 = y[0];
    //let last_y: u32 = *y.last().unwrap();

    let (a, b) = results
        .into_iter()
        .map(|p| both(|x| x.decrypt(&key_pair), &p.1))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    // println!("a={} b={}", a, b);
    let v = a as f64 / b as f64;
    let vms = (v * 1000.0).round() as u32;
    let dt = (first_y as f64 / v).round() as u32;
    let t0 = first_x + dt;
    println!("Approximate speed: {} m/s", vms);
    println!("Prognosed flight time: {} s", dt);
    let (hh, mm, ss) = sec_to_hms(t0);
    println!("Prognosed impact time: {:02}:{:02}:{:02}", hh, mm, ss);

    Ok(())
}

fn sec_to_hms(t: u32) -> (u32, u32, u32) {
    let (hh, t1) = (t / 3600, t % 3600);
    let (mm, ss) = (t1 / 60, t1 % 60);
    (hh, mm, ss)
}
