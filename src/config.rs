use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg()]
    pub script: String,
    
    #[arg()]
    pub files: Vec<String>,
}
