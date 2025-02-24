use structopt::StructOpt;

use lambers_w::input::{take_value, CLI};
use lambers_w::error::Error;
    
mod input;
mod error;

fn main() {
  env_logger::init();
  let opt = input::CLI::from_args();
  
  let output = input::take_value(opt.x);

  println!("the value of w is {:?}", output);
}