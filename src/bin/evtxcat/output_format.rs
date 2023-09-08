
#[derive(clap::ValueEnum, Clone)]
pub (crate) enum OutputFormat {
    Json,
    Xml,
}