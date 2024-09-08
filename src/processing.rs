use crate::config::Config;
use crate::input_source::InputSource;
use crate::output_target::OutputTarget;
use crate::parsing::{Script, Command, Substitution};

pub fn process_input(
    config: Config,
    script: Script,
    input_sources: Vec<Box<dyn InputSource>>,
    output: &mut dyn OutputTarget,
) -> Result<usize, String> {
    let match_count = 0;

    for input_source in input_sources {
        let mut input = input_source.reader();
        let mut line = String::new();

        while input.read_line(&mut line).unwrap() > 0 {
            for command in &script.commands {
                match command {
                    Command::Substitute(substitution) => {
                        line = substitute(&line, substitution);
                    }
                }
            }

            output.write(&line);
            line.clear();
        }
    }

    Ok(match_count)
}

fn substitute(line: &str, substitution: &Substitution) -> String {
    let pattern = regex::Regex::new(&substitution.pattern)
        .expect("Failed to compile pattern");
    let replacement = &substitution.replacement;
    let flags = substitution.flags.as_deref().unwrap_or("");

    pattern.replace_all(line, replacement).to_string()
}