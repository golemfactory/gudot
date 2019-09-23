use super::Result;
use gmorph::Enc;
use std::process;

pub(crate) fn exec(x: Vec<Enc>, y: Vec<Enc>) -> (Enc, Enc) {
    match exec_impl(x, y) {
        Ok(x_y) => x_y,
        Err(err) => {
            eprintln!("Error occurred while executing subtask: {}", err);
            process::exit(1);
        }
    }
}

fn exec_impl(x: Vec<Enc>, y: Vec<Enc>) -> Result<(Enc, Enc)> {
    let xy = dot_product_enc(&x, &y)?;
    let xx = dot_product_enc(&x, &x)?;
    Ok((xy, xx))
}

fn dot_product_enc(v: &Vec<Enc>, w: &Vec<Enc>) -> Result<Enc> {
    let length = v.len();
    if length <= 0 {
        return Err("Empty input vector!".into());
    }
    // We expect both vectors to have the same number of elements
    if length != w.len() {
        return Err("Vectors should have equal lengths!".into());
    }

    let mut sum = v[0] * w[0];
    for index in 1..length {
        sum = sum + v[index] * w[index];
    }
    Ok(sum)
}
