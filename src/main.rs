use gmorph::{Decrypt, Enc, Encrypt, KeyPair};
use gudot_utils::{deserialize_from_file, serialize_to_file};
use plotters::prelude::*;
use rand::prelude::*;
use rand_distr::Normal;
use structopt::StructOpt;

type Result<T> = std::result::Result<T, String>;

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

fn generate_impl() -> Result<()> {
    const FILENAME: &str = "input.json";

    //    let x = vec!(1,2,3,4);
    //    let y = vec!(2,4,6,8);
    let v = 2.71;
    let t0 = 9 * 3600;
    let d0 = 6624;

    let mut x = Vec::new();
    let mut y = Vec::new();

    let mut rng = thread_rng();
    let normal =
        Normal::new(0.0, 2.0).map_err(|err| format!("Couldn't create noise source: {:?}", err))?;

    for i in 1..100 {
        let t = t0 + i;
        let noise = normal.sample(&mut rng);
        let dd = (v * (i as f64) + noise).round() as u32;
        let d = d0 - dd;
        x.push(t);
        y.push(d);
    }

    serialize_to_file((x, y), FILENAME)
}

fn encrypt_impl() -> Result<()> {
    fn encrypt_vec(key_pair: &KeyPair, v: Vec<u32>) -> Vec<Enc> {
        v.into_iter().map(|x| Enc::encrypt(&key_pair, x)).collect()
    }

    const INPUT_FN: &str = "input.json";
    const KEYS_FN: &str = "keys.json";
    const OUTPUT_FN: &str = "enc_input.json";

    // input.json of the form
    // [[1,2,3],[2,4,6]]
    let key_pair = KeyPair::new();
    let (x, y): (Vec<u32>, Vec<u32>) = deserialize_from_file(INPUT_FN)?;

    let first_x: u32 = x[0];
    let first_y: u32 = y[0];
    let last_y = y
        .last()
        .ok_or("Expected at least one element in the input vector".to_string())?;

    let x1: Vec<u32> = x.into_iter().map(|v| v - first_x).collect();
    let y1: Vec<u32> = if *last_y < first_y {
        y.into_iter().map(|v| first_y - v).collect()
    } else {
        y.into_iter().map(|v| v - first_y).collect()
    };
    let enc_x = encrypt_vec(&key_pair, x1);
    let enc_y = encrypt_vec(&key_pair, y1);

    serialize_to_file((enc_x, enc_y), OUTPUT_FN)?;
    serialize_to_file(key_pair, KEYS_FN)
}

fn decrypt_impl() -> Result<()> {
    const KEYS_FN: &str = "keys.json";
    const INPUT_FN: &str = "enc_output.json";
    const OUTPUT_FN: &str = "output.json";

    let key_pair: KeyPair = deserialize_from_file(KEYS_FN)?;
    let data: Vec<(Enc, Enc)> = deserialize_from_file(INPUT_FN)?;

    // decrypt
    let data: Vec<_> = data
        .into_iter()
        .map(|(a, b)| (a.decrypt(&key_pair), b.decrypt(&key_pair)))
        .collect();

    serialize_to_file(data, OUTPUT_FN)
}

fn regress_impl() -> Result<()> {
    const INPUT_FN: &str = "input.json";
    const OUTPUT_FN: &str = "output.json";
    const REGRESS_FN: &str = "regress.json";

    let fitted: Vec<(u32, u32)> = deserialize_from_file(OUTPUT_FN)?;
    let (a, b) = fitted
        .into_iter()
        .fold((0, 0), |(acc_a, acc_b), (a, b)| (acc_a + a, acc_b + b));
    let slope = -1.0 * a as f64 / b as f64;

    let (x, y): (Vec<u32>, Vec<u32>) = deserialize_from_file(INPUT_FN)?;
    let (min_x, max_x) = (
        *x.iter().min().ok_or("Empty input vector x".to_string())? as f64,
        *x.iter().max().ok_or("Empty input vector x".to_string())? as f64,
    );
    let (min_y, max_y) = (
        *y.iter().min().ok_or("Empty input vector y".to_string())? as f64,
        *y.iter().max().ok_or("Empty input vector y".to_string())? as f64,
    );
    let intercept = (min_y + max_y - slope * (min_x + max_x)) / 2.0;

    println!("Fitted model: y = {}x + {}", slope, intercept);

    serialize_to_file((slope, intercept), REGRESS_FN)
}

fn plot_impl() -> Result<()> {
    const INPUT_FN: &str = "input.json";
    const REGRESS_FN: &str = "regress.json";
    const PLOT_FN: &str = "plot.png";

    let (x, y): (Vec<u32>, Vec<u32>) = deserialize_from_file(INPUT_FN)?;
    let (min_x, max_x) = (
        *x.iter().min().ok_or("Empty input vector x".to_string())? as f64,
        *x.iter().max().ok_or("Empty input vector x".to_string())? as f64,
    );
    let (min_y, max_y) = (
        *y.iter().min().ok_or("Empty input vector y".to_string())? as f64,
        *y.iter().max().ok_or("Empty input vector y".to_string())? as f64,
    );
    let points: Vec<(f64, f64)> = x
        .into_iter()
        .zip(y.into_iter())
        .map(|(x, y)| (x as f64, y as f64))
        .collect();

    let root = BitMapBackend::new(PLOT_FN, (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let root = root.margin(20, 20, 20, 20);
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_ranged(min_x..max_x, min_y..max_y)
        .unwrap();
    chart.configure_mesh().draw().unwrap();
    chart
        .draw_series(PointSeries::of_element(
            points.clone(),
            2,
            &BLUE,
            &|c, s, st| {
                return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
            },
        ))
        .unwrap();

    if let Ok((slope, intercept)) = deserialize_from_file::<(f64, f64), _>(REGRESS_FN) {
        let points: Vec<(f64, f64)> = points
            .into_iter()
            .map(|(x, _)| (x, slope * x + intercept))
            .collect();
        let style = ShapeStyle::from(&RED);
        // The println is here, so it is executed after reading REGRESS_FN
        println!("Writing {}", PLOT_FN);
        chart
            .draw_series(LineSeries::new(points, style.stroke_width(2)))
            .unwrap();
    }

    Ok(())
}
