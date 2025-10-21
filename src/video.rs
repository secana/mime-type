use crate::MimeFormat;
use std::fmt::{self, Display, Formatter};

/// Video file formats.
///
/// Supports common video container and codec formats.
pub enum Video {
    /// MPEG-4 Part 14 video format
    Mp4,
    /// iTunes video format
    M4v,
    /// Matroska video format
    Mkv,
    /// WebM video format
    Webm,
    /// QuickTime movie format
    Mov,
    /// Audio Video Interleave format
    Avi,
    /// Windows Media Video format
    Wmv,
    /// MPEG video format
    Mpg,
    /// Flash Video format
    Flv,
}

impl Display for Video {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mime_str = match self {
            Video::Mp4 => "video/mp4",
            Video::M4v => "video/x-m4v",
            Video::Mkv => "video/x-matroska",
            Video::Webm => "video/webm",
            Video::Mov => "video/quicktime",
            Video::Avi => "video/x-msvideo",
            Video::Wmv => "video/x-ms-wmv",
            Video::Mpg => "video/mpeg",
            Video::Flv => "video/x-flv",
        };
        write!(f, "{}", mime_str)
    }
}

impl MimeFormat for Video {
    fn from_ext(ext: &str) -> Option<crate::MimeType> {
        match ext {
            "mp4" => Some(crate::MimeType::Video(Video::Mp4)),
            "m4v" => Some(crate::MimeType::Video(Video::M4v)),
            "mkv" => Some(crate::MimeType::Video(Video::Mkv)),
            "webm" => Some(crate::MimeType::Video(Video::Webm)),
            "mov" => Some(crate::MimeType::Video(Video::Mov)),
            "avi" => Some(crate::MimeType::Video(Video::Avi)),
            "wmv" => Some(crate::MimeType::Video(Video::Wmv)),
            "mpg" | "mpeg" => Some(crate::MimeType::Video(Video::Mpg)),
            "flv" => Some(crate::MimeType::Video(Video::Flv)),
            _ => None,
        }
    }

    fn from_mime(mime: &str) -> Option<crate::MimeType> {
        match mime {
            "video/mp4" => Some(crate::MimeType::Video(Video::Mp4)),
            "video/x-m4v" => Some(crate::MimeType::Video(Video::M4v)),
            "video/x-matroska" => Some(crate::MimeType::Video(Video::Mkv)),
            "video/webm" => Some(crate::MimeType::Video(Video::Webm)),
            "video/quicktime" => Some(crate::MimeType::Video(Video::Mov)),
            "video/x-msvideo" => Some(crate::MimeType::Video(Video::Avi)),
            "video/x-ms-wmv" => Some(crate::MimeType::Video(Video::Wmv)),
            "video/mpeg" => Some(crate::MimeType::Video(Video::Mpg)),
            "video/x-flv" => Some(crate::MimeType::Video(Video::Flv)),
            _ => None,
        }
    }
}
