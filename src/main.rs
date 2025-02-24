use structopt::StructOpt;
mod input;
mod error;
use log::info;
fn main() {
  env_logger::init();
  info!("usher what is ya problem");
  let opt = input::CLI::from_args();
  
  let output = input::take_value(opt.x);

  println!("the value of w is {:?}", output);
}