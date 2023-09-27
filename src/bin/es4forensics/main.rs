mod cli;


use std::io::BufRead;
use anyhow::{Result, anyhow};

use cli::{Cli, Action};
use elasticsearch::auth::Credentials;
use dfir_toolkit::es4forensics::*;
use dfir_toolkit::common::{FancyParser, FileInput};

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Cli = Cli::parse_cli();
    
    let action = cli.action.clone();
    let e4f: Es4Forensics = cli.into();
    e4f.run(action).await
}

struct Es4Forensics {
    strict_mode: bool,
    index_name: String,
    host: String,
    port: u16,
    protocol: Protocol,
    omit_certificate_validation: bool,
    username: String,
    password: String,
}

impl Es4Forensics {
    pub async fn run(self, action: Action) -> Result<()> {

        let builder = self.create_index_builder()?;

        match action {
            Action::CreateIndex => {
                if builder.index_exists().await? {
                    return Err(anyhow!("index '{}' exists already", self.index_name));
                }
                builder.create_index().await?;
                Ok(())
            }
            Action::Import{input_file, bulk_size} => {
                self.import(builder, input_file.into(), bulk_size).await
            }
        }
    }

    async fn import(&self, builder: IndexBuilder, reader: FileInput, bulk_size: usize) -> Result<()> {
        let mut index = builder.connect().await?;
        index.set_cache_size(bulk_size).await?;

        for line in reader.lines() {
            let line = line?;
            let value = match serde_json::from_str(&line) {
                Ok(v) => v,
                Err(why) => {
                    if self.strict_mode {
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
        let mut builder = IndexBuilder::with_name(self.index_name.clone())
            .with_host(self.host.clone())
            .with_port(self.port)
            .with_credentials(Credentials::Basic(
                self.username.clone(),
                self.password.clone(),
            ))
            .with_protocol(self.protocol.clone());

        if self.omit_certificate_validation {
            ::log::warn!("disabling certificate validation");
            builder = builder.without_certificate_validation();
        }

        Ok(builder)
    }
}

impl From<Cli> for Es4Forensics {
    fn from(cli: Cli) -> Self {
        Self {
            strict_mode: cli.strict_mode,
            host: cli.host.clone(),
            port: cli.port,
            username: cli.username.clone(),
            password: cli.password.clone(),
            index_name: cli.index_name.clone(),
            protocol: cli.protocol.clone(),
            omit_certificate_validation: cli.omit_certificate_validation,
        }
    }
}