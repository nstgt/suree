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
    pub help_string: Option<String>,

    #[clap()]
    pub commands: Vec<String>,
}
