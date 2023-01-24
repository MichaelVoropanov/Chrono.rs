#![allow(unused)]

use clap::Parser;
use std::io;
use serde_derive::{Serialize, Deserialize};
use confy;
use human_panic::setup_panic;
// Constants
const CURRENT_VERSION: &str = "0.1.0";
const DEFAULT_THEME: &str = "default";

// CLI struct for clap arguments parsing
#[derive(Parser)]
struct Cli {
    mode : String,
    options: String,
    duration : String,

}
// Config struct for confy config file
#[derive(Serialize, Deserialize)]
struct MyConfig {
    version: String,
    theme: String
}

// Sets the default values for the config
impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { version: CURRENT_VERSION.into(), theme: DEFAULT_THEME.into() } }
}
// ===============================================================================================

fn main() -> Result<(), confy::ConfyError> {
    let cfg = confy::load("chrono", None)?;
    dbg!(cfg);
    println!("{:#?}", cfg);
    setup_panic!();
    panic!("OMG EVERYTHING IS ON FIRE!!!");
}
