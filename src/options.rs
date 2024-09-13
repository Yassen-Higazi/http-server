use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(version, about="HTTP Server", long_about = None)]
pub struct Options {
    #[arg(short = 'o', long, default_value = "localhost")]
    pub host: String,

    #[arg(short, long, default_value_t = 4221)]
    pub port: u16,

    #[arg(short = 'd', long = "directory", default_value = "/tmp")]
    pub files_directory: String,
}