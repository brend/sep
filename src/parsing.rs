use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;

pub struct Substitution {
    pub pattern: String,
    pub replacement: String,
    pub flags: Option<String>,
}

pub enum Command {
    Substitute(Substitution),
}

pub struct Script {
    pub commands: Vec<Command>,
}

#[derive(Debug)]
pub enum ParseError {
    UnknownRule,
}

#[derive(Parser)]
#[grammar = "sed.pest"] // Path to the grammar file
struct SedParser;

pub fn parse_script(script: &str) -> Result<Script, ParseError> {
    let pairs = SedParser::parse(Rule::script, script)
        .unwrap_or_else(|e| panic!("Failed to parse: {}", e));

    for pair in pairs {
        let script = analyze_script(pair.into_inner())?;
        return Ok(script);
    }

    panic!("Failed to parse the script");
}

fn analyze_script(pairs: pest::iterators::Pairs<Rule>) -> Result<Script, ParseError> {
    let mut commands = Vec::new();

    for pair in pairs {
        commands.push(analyze_command(pair)?);
    }

    Ok(Script { commands })
}

fn analyze_command(pair: Pair<Rule>) -> Result<Command, ParseError> {
    match pair.as_rule() {
        Rule::command => {
            let inner_pair = pair.into_inner().next().unwrap();
            let substitute = analyze_substitute(inner_pair)?;
            Ok(Command::Substitute(substitute))
        },
        _ => Err(ParseError::UnknownRule),
    }
}

fn analyze_substitute(pair: Pair<Rule>) -> Result<Substitution, ParseError> {
    match pair.as_rule() {
        Rule::substitute => {
            let mut inner_rules = pair.into_inner();

            // Extract the pattern, replacement, and flags
            let pattern = inner_rules.next().unwrap().as_str().to_string();
            let replacement = inner_rules.next().unwrap().as_str().to_string();
            let flags = inner_rules.next().map(|p| p.as_str().to_string());

            Ok(Substitution { pattern, replacement, flags })
        }
        _ => Err(ParseError::UnknownRule),
    }
}