use std::io::{BufRead, BufReader, Read};

use clio::Input;
use flate2::bufread::GzDecoder;

pub struct FileInput {
    stream: Box<dyn BufRead>,
    input: Input
}

impl From<Input> for FileInput {
    fn from(value: Input) -> Self {
        let cloned_input = value.clone();
        if let Some(extension) = value.path().extension() {
            if extension == "gz" {
                return Self {
                    stream: Box::new(BufReader::new(GzDecoder::new(BufReader::new(value)))),
                    input: cloned_input
                };
            }
        }

        Self {
            stream: Box::new(BufReader::new(value)),
            input: cloned_input
        }
    }
}

impl From<&Input> for FileInput {
    fn from(value: &Input) -> Self {
        Self::from(value.clone())
    }
}


impl From<&mut Input> for FileInput {
    fn from(value: &mut Input) -> Self {
        Self::from(value.clone())
    }
}

impl Clone for FileInput {
    fn clone(&self) -> Self {
        self.input.clone().into()
    }
}

impl Read for FileInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.stream.read(buf)
    }
}

impl BufRead for FileInput {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.stream.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.stream.consume(amt)
    }
}

impl<'a> TryFrom<&'a str> for FileInput {
    type Error = <Input as TryFrom<&'a str>>::Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let res = Input::try_from(value)?;
        Ok(res.into())
    }
}
