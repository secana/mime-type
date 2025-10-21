use crate::MimeFormat;
use std::fmt::{self, Display, Formatter};

pub enum Image {
    Jpeg,
    Png,
    Gif,
    Webp,
    Cr2,
    Tif,
    Bmp,
    Heif,
    Avif,
    Jxr,
    Psd,
    Ico,
    Ora,
    Djvu,
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mime_str = match self {
            Image::Jpeg => "image/jpeg",
            Image::Png => "image/png",
            Image::Gif => "image/gif",
            Image::Webp => "image/webp",
            Image::Cr2 => "image/x-canon-cr2",
            Image::Tif => "image/tiff",
            Image::Bmp => "image/bmp",
            Image::Heif => "image/heif",
            Image::Avif => "image/avif",
            Image::Jxr => "image/vnd.ms-photo",
            Image::Psd => "image/vnd.adobe.photoshop",
            Image::Ico => "image/vnd.microsoft.icon",
            Image::Ora => "image/openraster",
            Image::Djvu => "image/vnd.djvu",
        };
        write!(f, "{}", mime_str)
    }
}

impl MimeFormat for Image {
    fn from_ext(ext: &str) -> Option<crate::MimeType> {
        match ext {
            "jpg" | "jpeg" => Some(crate::MimeType::Image(Image::Jpeg)),
            "png" => Some(crate::MimeType::Image(Image::Png)),
            "gif" => Some(crate::MimeType::Image(Image::Gif)),
            "webp" => Some(crate::MimeType::Image(Image::Webp)),
            "cr2" => Some(crate::MimeType::Image(Image::Cr2)),
            "tif" | "tiff" => Some(crate::MimeType::Image(Image::Tif)),
            "bmp" => Some(crate::MimeType::Image(Image::Bmp)),
            "heif" => Some(crate::MimeType::Image(Image::Heif)),
            "avif" => Some(crate::MimeType::Image(Image::Avif)),
            "jxr" => Some(crate::MimeType::Image(Image::Jxr)),
            "psd" => Some(crate::MimeType::Image(Image::Psd)),
            "ico" => Some(crate::MimeType::Image(Image::Ico)),
            "ora" => Some(crate::MimeType::Image(Image::Ora)),
            "djvu" => Some(crate::MimeType::Image(Image::Djvu)),
            _ => None,
        }
    }

    fn from_mime(mime: &str) -> Option<crate::MimeType> {
        match mime {
            "image/jpeg" => Some(crate::MimeType::Image(Image::Jpeg)),
            "image/png" => Some(crate::MimeType::Image(Image::Png)),
            "image/gif" => Some(crate::MimeType::Image(Image::Gif)),
            "image/webp" => Some(crate::MimeType::Image(Image::Webp)),
            "image/x-canon-cr2" => Some(crate::MimeType::Image(Image::Cr2)),
            "image/tiff" => Some(crate::MimeType::Image(Image::Tif)),
            "image/bmp" => Some(crate::MimeType::Image(Image::Bmp)),
            "image/heif" => Some(crate::MimeType::Image(Image::Heif)),
            "image/avif" => Some(crate::MimeType::Image(Image::Avif)),
            "image/vnd.ms-photo" => Some(crate::MimeType::Image(Image::Jxr)),
            "image/vnd.adobe.photoshop" => Some(crate::MimeType::Image(Image::Psd)),
            "image/vnd.microsoft.icon" => Some(crate::MimeType::Image(Image::Ico)),
            "image/openraster" => Some(crate::MimeType::Image(Image::Ora)),
            "image/vnd.djvu" => Some(crate::MimeType::Image(Image::Djvu)),
            _ => None,
        }
    }
}
