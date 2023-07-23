mod cli;


use std::io::BufRead;
use anyhow::{Result, anyhow};

use cli::{Cli, Action};
use elasticsearch::auth::Credentials;
use dfir_toolkit::es4forensics::*;
use clap::Parser;
use simplelog::{TermLogger, Config, ColorChoice, TerminalMode};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let _ = TermLogger::init(
        cli.verbose.log_level_filter(),
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto);
        
    let e4f: Es4Forensics = cli.into();
    e4f.run().await
}

struct Es4Forensics {
    cli: Cli
}

impl Es4Forensics {
    pub async fn run(self) -> Result<()> {

        let builder = self.create_index_builder()?;

        match &self.cli.action {
            Action::CreateIndex => {
                if builder.index_exists().await? {
                    return Err(anyhow!("index '{}' exists already", self.cli.index_name));
                }
                builder.create_index().await?;
                Ok(())
            }
            Action::Import{input_file, bulk_size} => {
                let source = StreamSource::from(input_file)?;
                self.import(builder, source.into(), *bulk_size).await
            }
        }
    }

    async fn import(&self, builder: IndexBuilder, reader: Box<dyn BufRead + Send>, bulk_size: usize) -> Result<()> {
        let mut index = builder.connect().await?;
        index.set_cache_size(bulk_size).await?;

        for line in reader.lines() {
            let line = line?;
            let value = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(why) => {
                    if self.cli.strict_mode {
                        return Err(anyhow!(why))
                    } else {
                        ::log::error!("error while parsing: {}", why);
                        ::log::error!("failed JSON was:     {}", line);
                        continue;
                    }
                }
            };

            index.add_bulk_document(value).await?;
        }
        index.flush().await?;
        Ok(())
    }    

    fn create_index_builder(&self) -> Result<IndexBuilder> {
        let mut builder = IndexBuilder::with_name(self.cli.index_name.clone())
            .with_host(self.cli.host.clone())
            .with_port(self.cli.port)
            .with_credentials(Credentials::Basic(
                self.cli.username.clone(),
                self.cli.password.clone(),
            ))
            .with_protocol(self.cli.protocol.clone());

        if self.cli.omit_certificate_validation {
            ::log::warn!("disabling certificate validation");
            builder = builder.without_certificate_validation();
        }

        Ok(builder)
    }
}

// 0039 035958041 (146)

impl From<Cli> for Es4Forensics {
    fn from(cli: Cli) -> Self {
        Self {
            cli
        }
    }
}