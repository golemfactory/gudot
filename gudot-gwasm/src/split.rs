use super::utils::{both, chop, zip_pair};
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

fn split_impl(num_subtasks: usize) -> Result<Vec<(Vec<Enc>, Vec<Enc>)>, String> {
    let mut data_file =
        File::open("data.json").map_err(|_| "Failed to open data.json".to_string())?;
    let mut serialized = String::new();
    data_file
        .read_to_string(&mut serialized)
        .map_err(|_| "Couldn't read data.json to String".to_string())?;
    let data: (Vec<Enc>, Vec<Enc>) = serde_json::from_str(&serialized)
        .map_err(|err| format!("Invalid JSON found in data.json: {}", err))?;
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
