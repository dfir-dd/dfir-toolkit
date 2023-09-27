use std::io::Read;

use clio::Input;
#[cfg(feature = "gzip")]
use flate2::read::GzDecoder;
pub(crate) struct StreamSource(Box<dyn Read + Send>);

impl From<Input> for StreamSource {
    fn from(input: Input) -> Self {
        #[cfg(feature = "gzip")]
        if input.path().ends_with(".gz") {
            return Self(Box::new(GzDecoder::new(input)));
        }

        Self(Box::new(input))
    }
}

impl Read for StreamSource {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}