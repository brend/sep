mod config;
mod input_source;
mod output_target;
mod parsing;
mod processing;

use clap::Parser;
use glob::glob;
use crate::config::Config;
use crate::input_source::{FileInputSource, InputSource};
use crate::output_target::StdoutTarget;
use crate::parsing::parse_script;
use crate::processing::process_input;

fn main() {
    let config = Config::parse();
    let script = parse_script(&config.script).expect("Failed to parse script");
    let mut input_sources: Vec<Box<dyn InputSource>> = Vec::new();

    if config.files.is_empty() {
        // If no files are specified, read from stdin
        input_sources.push(Box::new(FileInputSource {
            filename: "-".to_string(),
        }));
    } else {
        // Otherwise, read from the specified files
        for pattern in &config.files {
            // Expand globs in the file patterns
            let matches = glob(pattern).expect("Failed to read glob pattern");
            for entry in matches {
                match entry {
                    Ok(path) => {
                        input_sources.push(Box::new(FileInputSource {
                            filename: path.to_string_lossy().to_string(),
                        }));
                    }
                    Err(e) => {
                        eprintln!("Error matching pattern {}: {}", pattern, e);
                        std::process::exit(2);
                    }
                }
            }
        }
    }

    let mut output = StdoutTarget;

    match process_input(config, script, input_sources, &mut output) {
        Ok(match_count) => {
            std::process::exit(if match_count > 0 { 0 } else { 1 });
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(2);
        }
    }
}