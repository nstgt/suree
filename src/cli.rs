use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(
        short = 'H',
        long = "help-string",
        value_name = "HELP STRING",
        default_value = "--help",
        allow_hyphen_values = true
    )]
    pub help_string: String,

    #[clap(required = true)]
    pub commands: Vec<String>,
}
