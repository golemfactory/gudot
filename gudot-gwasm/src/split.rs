use super::Result;
use gmorph::Enc;
use gudot_utils::{both, chop, deserialize_from_file, zip_pair};
use gwasm_api::SplitContext;
use serde::{Deserialize, Serialize};
use std::process;
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
    const FILENAME: &str = "enc_input.json";
    let data: (Vec<Enc>, Vec<Enc>) = deserialize_from_file(FILENAME)?;
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
