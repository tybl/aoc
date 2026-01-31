use clap::Parser;

#[derive(Parser)]
pub struct CliArguments {
    #[arg(short, long, default_value = "input/0.txt")]
    pub input: String,
}
