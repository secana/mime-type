use crate::MimeType;

/// Trait for converting between file extensions, MIME strings, and MIME types.
///
/// Implemented by all MIME type categories to provide bidirectional conversion.
pub trait MimeFormat {
    /// Converts a file extension to a MIME type.
    ///
    /// Returns `None` if the extension is not recognized.
    ///
    /// # Examples
    ///
    /// ```
    /// use mime_type::{MimeType, MimeFormat};
    ///
    /// let mime = MimeType::from_ext("png");
    /// assert!(mime.is_some());
    /// ```
    fn from_ext(ext: &str) -> Option<MimeType>;

    /// Converts a MIME type string to a MIME type.
    ///
    /// Returns `None` if the MIME string is not recognized.
    ///
    /// # Examples
    ///
    /// ```
    /// use mime_type::{MimeType, MimeFormat};
    ///
    /// let mime = MimeType::from_mime("image/png");
    /// assert!(mime.is_some());
    /// ```
    fn from_mime(mime: &str) -> Option<MimeType>;
}
