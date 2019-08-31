use clap::{App, Arg};
use std::env::current_dir;
use std::path::PathBuf;
use std::fs::canonicalize;

mod generator;

fn main() {
  let args = App::new("granex-ucode")
    .version(env!("CARGO_PKG_VERSION"))
    .author("Vitaly Domnikov <oss@vitaly.codes> & Lukas Friedrich <lukas.b.friedrich@gmail.com>")
    .about("uCode Tor v3 Vanity Address Generator")
    .arg(
      Arg::with_name("prefix")
        .short("p")
        .long("prefix")
        .help("Sets the address prefix")
        .takes_value(true)
        .required(true)
        .display_order(0)
        .index(1),
    )
    .arg(
      Arg::with_name("output")
        .short("o")
        .long("output")
        .help("Output directory")
        .takes_value(true)
        .display_order(1)
        .index(2),
    )
    .arg(
      Arg::with_name("threads")
        .short("t")
        .long("threads")
        .help("Number of CPU Threads")
        .takes_value(true)
        .display_order(2)
        .index(3)
    )
    .get_matches();
  let prefix = args.value_of("prefix").unwrap();
  let output_dir = match args.value_of("output") {
    Some(val) => canonicalize(&PathBuf::from(val)).unwrap(),
    None => current_dir().unwrap(),
  };
  let threads = match args.value_of("threads") {
    Some(val) => val,
    None => "4"
    
  };
  let thrc = threads.parse::<i32>().unwrap();
  generator::start(prefix, output_dir, thrc).unwrap();
}
