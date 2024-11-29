use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
pub struct GenpassOpts {
    #[arg(short, long, help = "Length of password", default_value = "16")]
    pub length: u8,
    #[arg(long, help = "With uppercase in the password", action = ArgAction::Set, default_value_t = true)]
    pub uppercase: bool,
    #[arg(long, help = "With lowercase in the password", action = ArgAction::Set, default_value_t = true)]
    pub lowercase: bool,
    #[arg(long, help = "With number in the password", action = ArgAction::Set, default_value_t = true)]
    pub number: bool,
    #[arg(
        long,
        help = "With special symbol in the password",
        action = ArgAction::Set,
        default_value_t = true
    )]
    pub symbol: bool,
}
