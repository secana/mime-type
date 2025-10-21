//! MIME type definitions and conversions for Rust.
//!
//! This crate provides type-safe MIME type handling with support for converting
//! between file extensions and MIME type strings.
//!
//! # Examples
//!
//! ```
//! use mime_type::{MimeType, MimeFormat};
//!
//! // Get MIME type from file extension
//! let mime = MimeType::from_ext("png").unwrap();
//! assert_eq!(mime.to_string(), "image/png");
//!
//! // Get MIME type from MIME string
//! let mime = MimeType::from_mime("video/mp4").unwrap();
//! ```

mod application;
mod archive;
mod audio;
mod book;
mod document;
mod font;
mod image;
mod mime_format;
mod video;

use std::fmt::{self, Display, Formatter};

pub use application::Application;
pub use archive::Archive;
pub use audio::Audio;
pub use book::Book;
pub use document::Document;
pub use font::Font;
pub use image::Image;
pub use mime_format::MimeFormat;
pub use video::Video;

/// Main MIME type enum containing all supported categories.
///
/// This enum wraps the specific MIME type categories (Image, Video, Audio, etc.)
/// and implements conversion from file extensions and MIME strings.
pub enum MimeType {
    /// Image formats (JPEG, PNG, GIF, etc.)
    Image(Image),
    /// Video formats (MP4, MKV, WebM, etc.)
    Video(Video),
    /// Audio formats (MP3, FLAC, WAV, etc.)
    Audio(Audio),
    /// Archive and compressed formats (ZIP, TAR, RAR, etc.)
    Archive(Archive),
    /// E-book formats (EPUB, MOBI)
    Book(Book),
    /// Document formats (DOC, PDF, XLS, etc.)
    Document(Document),
    /// Font formats (TTF, OTF, WOFF, etc.)
    Font(Font),
    /// Application and executable formats (EXE, WASM, ELF, etc.)
    Application(Application),
}

impl Display for MimeType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MimeType::Image(img) => write!(f, "{}", img),
            MimeType::Video(vid) => write!(f, "{}", vid),
            MimeType::Audio(aud) => write!(f, "{}", aud),
            MimeType::Archive(arch) => write!(f, "{}", arch),
            MimeType::Book(book) => write!(f, "{}", book),
            MimeType::Document(doc) => write!(f, "{}", doc),
            MimeType::Font(font) => write!(f, "{}", font),
            MimeType::Application(app) => write!(f, "{}", app),
        }
    }
}

impl MimeFormat for MimeType {
    fn from_ext(ext: &str) -> Option<MimeType> {
        Image::from_ext(ext)
            .or_else(|| Video::from_ext(ext))
            .or_else(|| Audio::from_ext(ext))
            .or_else(|| Archive::from_ext(ext))
            .or_else(|| Book::from_ext(ext))
            .or_else(|| Document::from_ext(ext))
            .or_else(|| Font::from_ext(ext))
            .or_else(|| Application::from_ext(ext))
    }

    fn from_mime(mime: &str) -> Option<MimeType> {
        Image::from_mime(mime)
            .or_else(|| Video::from_mime(mime))
            .or_else(|| Audio::from_mime(mime))
            .or_else(|| Archive::from_mime(mime))
            .or_else(|| Book::from_mime(mime))
            .or_else(|| Document::from_mime(mime))
            .or_else(|| Font::from_mime(mime))
            .or_else(|| Application::from_mime(mime))
    }
}
