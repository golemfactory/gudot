use super::Result;
use gmorph::Enc;
use gwasm_api::SplitContext;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, process};
use structopt::StructOpt;

pub(crate) fn split(context: &mut dyn SplitContext) -> Vec<(Vec<Enc>, Vec<Enc>)> {
    let params = parse_args::<GudotParams>(context.args());
    let num_subtasks = params.num_subtasks;

    match split_impl(num_subtasks) {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("Error occurred while splitting into subtasks: {}", err);
            process::exit(1);
        }
    }
}

fn split_impl(num_subtasks: usize) -> Result<Vec<(Vec<Enc>, Vec<Enc>)>> {
    let filename = "enc_input.json";
    let mut data_file = File::open(&filename)
        .map_err(|err| format!("Failed to open file {}: {}", &filename, err))?;
    let mut serialized = String::new();
    data_file
        .read_to_string(&mut serialized)
        .map_err(|err| format!("Couldn't read {} to String: {}", &filename, err))?;
    let data: (Vec<Enc>, Vec<Enc>) = serde_json::from_str(&serialized)
        .map_err(|err| format!("Invalid JSON found in {}: {}", &filename, err))?;
    Ok(zip_pair(both(|v| chop(v, num_subtasks), &data)).collect())
}

#[derive(Debug, StructOpt, Clone, Serialize, Deserialize)]
struct GudotParams {
    #[structopt(long = "subtasks", default_value = "1")]
    num_subtasks: usize,
}

fn parse_args<ArgsType: StructOpt>(args: &Vec<String>) -> ArgsType {
    // Note: StructOpt wants to eat first argument as program name.
    // But we don't have program name at this point, so we must add fake argument.
    let mut args_copy = args.clone();
    args_copy.insert(0, String::from("--"));

    return ArgsType::from_iter(args_copy.into_iter());
}

/// Chop as vector into `count` chunks, returning an iterator
fn chop<'a>(v: &'a Vec<Enc>, count: usize) -> impl Iterator<Item = Vec<Enc>> + 'a {
    v.chunks(v.len() / count).map(|c| c.to_vec())
}

/// Apply a function to both components of a pair
fn both<'a, A, B>(f: impl Fn(&'a A) -> B, pair: &'a (A, A)) -> (B, B) {
    (f(&pair.0), f(&pair.1))
}

/// Zip a pair of iterators, yielding a iterator of pairs
/// zip_pair :: ([a], [b]) -> [(a,b)]
fn zip_pair<'a, A, B>(
    pair: (impl Iterator<Item = A> + 'a, impl Iterator<Item = B> + 'a),
) -> impl Iterator<Item = (A, B)> + 'a {
    pair.0.zip(pair.1)
}
