use crate::MimeType;

pub trait MimeFormat {
    fn from_ext(ext: &str) -> Option<MimeType>;
    fn from_mime(mime: &str) -> Option<MimeType>;
}
