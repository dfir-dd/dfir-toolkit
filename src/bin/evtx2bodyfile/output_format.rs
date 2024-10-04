use clap::ValueEnum;
use strum_macros::Display;

#[derive(ValueEnum, Clone, Display)]
pub(crate) enum OutputFormat {
    
    /// bodyfile format
    #[strum(serialize = "bodyfile")]
    Bodyfile,

    /// flow record format (<https://docs.rs/flow-record>)
    #[strum(serialize = "record")]
    Record
}
