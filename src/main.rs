use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use regex::Regex;

#[derive(Parser)]
#[grammar = "sed.pest"] // Path to the grammar file
struct SedParser;

fn main() {
    let input_script = "s/foo/bar/g";  // Example sed-like script
    let input_text = "The word foo is often used as a metavariable";    // Example text to apply the substitution to

    println!("Input script: {}", input_script);
    println!("Input text: {}", input_text);

    // Parse the input script
    let pairs = SedParser::parse(Rule::script, input_script)
        .unwrap_or_else(|e| panic!("Failed to parse: {}", e));

    // Process each command in the script
    for pair in pairs {
        process_script(pair, input_text);
    }
}

// Process the parsed `script` rule
fn process_script(pair: Pair<Rule>, text: &str) {
    match pair.as_rule() {
        Rule::script => {
            for inner_pair in pair.into_inner() {
                process_command(inner_pair, text);
            }
        },
        _ => unreachable!(),
    }
}

// Process the parsed `command` rule
fn process_command(pair: Pair<Rule>, text: &str) {
    match pair.as_rule() {
        Rule::command => {
            let inner_pair = pair.into_inner().next().unwrap();
            process_substitute(inner_pair, text);
        },
        _ => unreachable!(),
    }
}

// Process the parsed `substitute` rule
fn process_substitute(pair: Pair<Rule>, text: &str) {
    match pair.as_rule() {
        Rule::substitute => {
            let mut inner_rules = pair.into_inner();

            // Extract the pattern, replacement, and flags
            let pattern = inner_rules.next().unwrap().as_str();
            let replacement = inner_rules.next().unwrap().as_str();
            let flags = inner_rules.next().map(|p| p.as_str());

            println!("Parse result: Substitute command");
            println!("Pattern: {}", pattern);
            println!("Replacement: {}", replacement);
            if let Some(flag) = flags {
                println!("Flags: {}", flag);
            }

            // Apply the substitution to the text
            let result = apply_substitution(pattern, replacement, flags, text);
            println!("Resulting text: {}", result);
        }
        _ => unreachable!(),
    }
}

// Function to apply the substitution using regex
fn apply_substitution(pattern: &str, replacement: &str, flags: Option<&str>, text: &str) -> String {
    // Build the regex with the pattern
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(err) => panic!("Invalid regex pattern: {}", err),
    };

    // Check if we have the global flag 'g'
    if let Some(flag) = flags {
        if flag.contains('g') {
            // Global replacement
            return re.replace_all(text, replacement).to_string();
        }
    }

    // Single replacement (no global flag)
    re.replace(text, replacement).to_string()
}