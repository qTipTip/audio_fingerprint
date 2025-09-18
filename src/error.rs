use hound;
#[derive(Debug)]
pub enum AudioError {
    Hound(hound::Error),
    UnsupportedFormat(hound::SampleFormat, u16),
}

impl From<hound::Error> for AudioError {
    fn from(err: hound::Error) -> Self {
        AudioError::Hound(err)
    }
}
