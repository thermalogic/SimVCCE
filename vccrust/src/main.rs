//! Main program for vccrust

use vccrust::JSONLoader;
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
                    curcycle.state();
                    curcycle.balance();
                    curcycle.outresults();
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
