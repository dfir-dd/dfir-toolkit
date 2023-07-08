use std::{io::{Read, BufReader}, fs::File};
use anyhow::Result;

#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;

pub (crate) enum StreamSource {
    Stdin,
    File(Box<dyn Read + Send>),
}

impl StreamSource {
    pub fn from(filename: &Option<String>) -> Result<Self> {
        match filename {
            None => Ok(StreamSource::Stdin),
            Some(filename) =>  {
                if filename == "-" { Ok(StreamSource::Stdin) }
                else {
                    let file = BufReader::new(File::open(filename)?);

                    #[cfg(not(feature = "gzip"))]
                    let reader: Box<dyn BufRead> = Box::new(file);

                    #[cfg(feature = "gzip")]
                    let reader = Self::open_gzip(filename, file);

                    Ok(StreamSource::File(reader))
                }
            }
        }
    }

    #[cfg(feature = "gzip")]
    fn open_gzip<R: Read + Send + 'static>(filename: &str, file: R) -> Box<dyn Read + Send> {
        if filename.ends_with(".gz") {
            Box::new(GzDecoder::new(file))
        } else {
            Box::new(file)
        }
    }
}