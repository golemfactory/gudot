pub mod utils;

use gmorph::*;
use gwasm_api::dispatcher;
// use gwasm_api::{Blob, Output, TaskResult};
use gwasm_api::SplitContext;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use structopt::*;


#[derive(Debug, StructOpt, Clone, Serialize, Deserialize)]
pub struct GudotParams {
    #[structopt(long = "subtasks", default_value = "1")]
    num_subtasks: usize,
}


fn split(context: &mut dyn SplitContext) -> Vec<(Vec<Enc>, Vec<Enc>)> {
    let params = utils::parse_args::<GudotParams>(context.args());
    let num_subtasks = params.num_subtasks;

    let mut data_file = File::open("data.json").expect("Failed to open data.json");
    let mut serialized = String::new();
    data_file.read_to_string(&mut serialized).unwrap();
    let data: (Vec<Enc>, Vec<Enc>) = serde_json::from_str(&serialized).unwrap();
    zip_pair(both(|v| chop(v, num_subtasks), &data)).collect()
}


/// Chop as vector into `count` chunks, returning an iterator
fn chop<'a>(v: &'a Vec<Enc>, count: usize) -> impl Iterator<Item=Vec<Enc>> +'a {
   v.chunks(v.len() / count).map(|c| c.to_vec())
}

/// Apply a function to both components of a pair
fn both<'a, A, B>(f: impl Fn(&'a A) -> B, pair: &'a (A, A)) -> (B, B) {
    (f(&pair.0), f(&pair.1))
}

/// Zip a pair of iterators, yielding a iterator of pairs
/// zip_pair :: ([a], [b]) -> [(a,b)]
fn zip_pair<'a, A, B>(pair: (impl Iterator<Item=A> +'a,impl Iterator<Item=B> +'a))
                      -> impl Iterator<Item=(A,B)> +'a {
    pair.0.zip(pair.1)
}

fn execute(x: Vec<Enc>, y: Vec<Enc>) -> (Enc, Enc) {
    let xy = dot_product_enc(&x, &y);
    let xx = dot_product_enc(&x, &x);
    (xy, xx)
}


fn merge(_args: &Vec<String>, results: Vec<((Vec<Enc>, Vec<Enc>), (Enc, Enc))>) {
    let mut keys_file = File::open("keys.json").unwrap();
    let mut serialized_keypair = String::new();
    keys_file.read_to_string(&mut serialized_keypair).unwrap();
    let key_pair: KeyPair = serde_json::from_str(&serialized_keypair).unwrap();

    let (a, b) = results
        .into_iter()
        .map(|p| both(|x| x.decrypt(&key_pair), &p.1))
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    // println!("a={} b={}", a, b);
    let m = a as f64 / b as f64;
    println!("m = {}", m);
}

fn main() {
    dispatcher::run(&split, &execute, &merge).unwrap();
}

fn dot_product_enc(v: &Vec<Enc>, w: &Vec<Enc>) -> Enc {
    let length = v.len();
    // We expect both vectors to have the same number of elements
    assert_eq!(length, w.len());
    assert!(length > 0);

    let mut sum = v[0] * w[0];

    for index in 1..length {
        sum = sum + v[index] * w[index];
    }
    sum
}
