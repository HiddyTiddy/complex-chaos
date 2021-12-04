use std::fs::File;
use std::io::{Error, Write};

use num::Complex;
const FIRST_TERM: Complex<f64> = Complex::new(0.2, 0.0);
const ITERATIONS: usize = 1000;

const END_VALUES: usize = 100;
const STARTING_ELEMENT_INDEX: usize = ITERATIONS - END_VALUES;

// fn complex_round(x: Complex<f64>) -> Complex<f64> {
//     if x.re.is_nan() || x.im.is_nan() {
//         Complex::new(f64::NAN, f64::NAN)
//     } else {
//         Complex::new((x.re * 100.).round() / 100., (x.im * 100.) / 100.)
//     }
// }

fn series(lamb: Complex<f64>, x: Complex<f64>) -> Complex<f64> {
    lamb * x * (1. - x)
}

fn element(lamb: Complex<f64>, n: usize) -> Complex<f64> {
    if n == 0 {
        FIRST_TERM
    } else {
        let prev = element(lamb, n - 1);
        lamb * prev * (Complex::new(1.0, 0.0) - prev)
    }
}

// fn to_hashable(x: Complex<f64>) -> u128 {
//     if x.im.is_nan() || x.re.is_nan() {
//         u128::MAX
//     } else {
//         (x.im.to_bits() as u128) << 64 | (x.re.to_bits() as u128)
//     }
// }

fn counter(l: Vec<Complex<f64>>) -> u8 {
    let last = l[l.len() - 1];
    let prev = l[l.len() - 2];
    if last.is_nan() {
        101 // diverges (to sth crazy)
    } else if (last.im - prev.im).abs() - (last.re - prev.re).abs() < 0.01 {
        let mut count = 1;
        for i in 2..l.len() {
            let prev = l[l.len() - i];
            if (last.im - prev.im).abs() - (last.re - prev.re).abs() < 0.01 {
                count += 1;
            } else {
                break;
            }
        }
        101 - count + 40
    } else {
        101 // doesnt converge
    }
}

fn chaos(lamb: Complex<f64>) -> u8 {
    let mut series_l = vec![element(lamb, STARTING_ELEMENT_INDEX)];
    for k in 0..END_VALUES {
        series_l.push(series(lamb, series_l[k]));
    }
    counter(series_l)
}

fn main() -> Result<(), Error> {
    const VERTICAL_PRECISION: usize = 1549;
    const HORIZONTAL_PRECISION: usize = 4000;

    let mut im_array = vec![vec![0u8; HORIZONTAL_PRECISION]; VERTICAL_PRECISION];
    const TOP_LEFT: Complex<f64> = Complex::new(-2.1, 1.2);
    const BOTTOM_RIGHT: Complex<f64> = Complex::new(4.1, -1.2);

    const VERTICAL_STEP: f64 = (TOP_LEFT.im - BOTTOM_RIGHT.im) / (VERTICAL_PRECISION as f64);
    const HORIZONTALL_STEP: f64 = (BOTTOM_RIGHT.re - TOP_LEFT.re) / (HORIZONTAL_PRECISION as f64);

    for i in 0..VERTICAL_PRECISION {
        let lamb = TOP_LEFT.im - (VERTICAL_STEP * (i as f64));
        //lamb_imaginary = max_imag_lamb - vertical_step_length*i
        for j in 0..HORIZONTAL_PRECISION {
            im_array[i][j] = chaos(Complex::new(
                TOP_LEFT.re + HORIZONTALL_STEP * (j as f64),
                lamb,
            ));
        }
    }

    let mut output = File::create("dump")?;
    for i in 0..VERTICAL_PRECISION {
        let mut line = "".to_string();
        for j in 0..HORIZONTAL_PRECISION {
            line += &format!("{},", im_array[i][j]);
        }
        writeln!(output, "{}", line)?;
    }
    Ok(())
}
