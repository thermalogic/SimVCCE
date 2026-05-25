//! Main program entry point for simvcc.
//!
//! Loads a JSON configuration file, creates a VCCycle, runs the simulation,
//! and outputs the results.
//!
//! # Usage
//! ```bash
//! simvcc [json_file]
//! ```
//!
//! If no JSON file is specified, defaults to `jsonmodel/demovcc.json`.

use simvcc::JSONLoader;
use std::env;

fn main() {
    let json_file = env::args().nth(1).unwrap_or_else(|| "jsonmodel/demovcc.json".to_string());

    println!("Loading cycle from: {}", json_file);

    let loader = JSONLoader::new();
    match loader.load_file(&json_file) {
        Ok(json_value) => {
            match loader.create_cycle(&json_value) {
                Ok(mut curcycle) => {
                    println!("Successfully loaded cycle");
                    curcycle.simulator();
                    curcycle.out_results();
                }
                Err(e) => {
                    eprintln!("Error creating cycle: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading JSON file: {}", e);
        }
    }
}
