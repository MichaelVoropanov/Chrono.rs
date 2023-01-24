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

// ===============================================================================================

fn main() -> Result<(), confy::ConfyError> {
    let cfg = confy::load("chrono", None )?;
    dbg!(cfg);
    println!("{:#?}", cfg);

    setup_panic!();
    panic!("OMG EVERYTHING IS ON FIRE!!!");
}
