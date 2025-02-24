use std::f64::consts::E;
use crate::error::Error;
use log::info;
use structopt::StructOpt;


#[derive(StructOpt, Debug)]
#[structopt(name = "take_value")]
pub  struct CLI {
    #[structopt(short = "x", long = "input")]
   pub x: f64,
}

pub fn take_value(x:f64) -> Result<f64, Error> {
                          

    let div = -1.0 / E;


    if x < div {
        return Err(Error::Invalidinput);
    }

    if x > 0.0 {
        return Err(Error::ZeroValue);
    }

    let  w = x;
   println!("{}",x);

    let iter = 50;
    let tolerance = 1e-10;
    let mut wl = 0.0;
    for i in 0..iter {
        let f = w * E.powf(w) - x;

        let f_prime = E.powf(w) * w + E.powf(w);

        wl = w - f / f_prime;

        if (wl - w).abs() < tolerance {
            wl = w
        }
        // println!("Iteration {}: w = {}, f(w) = {}, f'(w) = {}", i, w, f, f_prime);
    }
    Ok(wl)
}

