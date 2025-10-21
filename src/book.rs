use crate::MimeFormat;
use std::fmt::{self, Display, Formatter};

pub enum Book {
    Epub,
    Mobi,
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mime_str = match self {
            Book::Epub => "application/epub+zip",
            Book::Mobi => "application/x-mobipocket-ebook",
        };
        write!(f, "{}", mime_str)
    }
}

impl MimeFormat for Book {
    fn from_ext(ext: &str) -> Option<crate::MimeType> {
        match ext {
            "epub" => Some(crate::MimeType::Book(Book::Epub)),
            "mobi" => Some(crate::MimeType::Book(Book::Mobi)),
            _ => None,
        }
    }

    fn from_mime(mime: &str) -> Option<crate::MimeType> {
        match mime {
            "application/epub+zip" => Some(crate::MimeType::Book(Book::Epub)),
            "application/x-mobipocket-ebook" => Some(crate::MimeType::Book(Book::Mobi)),
            _ => None,
        }
    }
}
