use crate::MimeFormat;
use std::fmt::{self, Display, Formatter};

/// Font file formats.
///
/// Supports common web and desktop font formats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Font {
    /// TrueType Font format
    Ttf,
    /// OpenType Font format
    Otf,
    /// Web Open Font Format
    Woff,
    /// Web Open Font Format 2
    Woff2,
}

impl Display for Font {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mime_str = match self {
            Font::Ttf => "application/font-sfnt",
            Font::Otf => "application/font-sfnt",
            Font::Woff => "application/font-woff",
            Font::Woff2 => "application/font-woff",
        };
        write!(f, "{}", mime_str)
    }
}

impl MimeFormat for Font {
    fn from_ext(ext: &str) -> Option<crate::MimeType> {
        match ext {
            "ttf" => Some(crate::MimeType::Font(Font::Ttf)),
            "otf" => Some(crate::MimeType::Font(Font::Otf)),
            "woff" => Some(crate::MimeType::Font(Font::Woff)),
            "woff2" => Some(crate::MimeType::Font(Font::Woff2)),
            _ => None,
        }
    }

    fn from_mime(mime: &str) -> Option<crate::MimeType> {
        match mime {
            "application/font-sfnt" => Some(crate::MimeType::Font(Font::Ttf)),
            "application/font-woff" => Some(crate::MimeType::Font(Font::Woff)),
            _ => None,
        }
    }
}
