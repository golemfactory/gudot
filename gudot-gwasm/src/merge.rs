use super::Result;
use gmorph::Enc;
use gudot_utils::serialize_to_file;
use std::process;

pub(crate) fn merge(_args: &Vec<String>, results: Vec<((Vec<Enc>, Vec<Enc>), (Enc, Enc))>) {
    if let Err(err) = merge_impl(results) {
        eprintln!("Error occurred while merging results: {}", err);
        process::exit(1);
    }
}

fn merge_impl(results: Vec<((Vec<Enc>, Vec<Enc>), (Enc, Enc))>) -> Result<()> {
    const FILENAME: &str = "enc_output.json";
    let results: Vec<_> = results.into_iter().map(|(_, p)| p).collect();
    serialize_to_file(results, FILENAME)
}
