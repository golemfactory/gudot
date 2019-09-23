use super::Result;
use gmorph::Enc;
use std::{fs::File, io::Write, process};

pub(crate) fn merge(_args: &Vec<String>, results: Vec<((Vec<Enc>, Vec<Enc>), (Enc, Enc))>) {
    if let Err(err) = merge_impl(results) {
        eprintln!("Error occurred while merging results: {}", err);
        process::exit(1);
    }
}

fn merge_impl(results: Vec<((Vec<Enc>, Vec<Enc>), (Enc, Enc))>) -> Result<()> {
    let filename = "enc_output.json";
    let mut file = File::create(&filename)
        .map_err(|err| format!("Failed to create file {}: {}", &filename, err))?;
    let results: Vec<_> = results.into_iter().map(|(_, p)| p).collect();
    let serialized = serde_json::to_string(&results)
        .map_err(|err| format!("Couldn't serialize output to JSON: {}", err))?;
    file.write_all(serialized.as_bytes())
        .map_err(|err| format!("Couldn't write output JSON to file {}: {}", &filename, err))
}
