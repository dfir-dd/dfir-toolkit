use clap::ValueEnum;
use strum_macros::Display;

#[derive(ValueEnum, Clone, Display)]
pub(crate) enum OutputFormat {
    Json,
    Bodyfile,
}
