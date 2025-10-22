use crate::MimeFormat;
use std::fmt::{self, Display, Formatter};

/// Document file formats.
///
/// Supports Microsoft Office and OpenDocument formats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Document {
    /// Microsoft Word document
    Doc,
    /// Microsoft Word document (Office Open XML)
    Docx,
    /// Microsoft Excel spreadsheet
    Xls,
    /// Microsoft Excel spreadsheet (Office Open XML)
    Xlsx,
    /// Microsoft PowerPoint presentation
    Ppt,
    /// Microsoft PowerPoint presentation (Office Open XML)
    Pptx,
    /// OpenDocument text document
    Odt,
    /// OpenDocument spreadsheet
    Ods,
    /// OpenDocument presentation
    Odp,
}

impl Display for Document {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mime_str = match self {
            Document::Doc => "application/msword",
            Document::Docx => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
            Document::Xls => "application/vnd.ms-excel",
            Document::Xlsx => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            Document::Ppt => "application/vnd.ms-powerpoint",
            Document::Pptx => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            Document::Odt => "application/vnd.oasis.opendocument.text",
            Document::Ods => "application/vnd.oasis.opendocument.spreadsheet",
            Document::Odp => "application/vnd.oasis.opendocument.presentation",
        };
        write!(f, "{}", mime_str)
    }
}

impl MimeFormat for Document {
    fn from_ext(ext: &str) -> Option<crate::MimeType> {
        match ext {
            "doc" => Some(crate::MimeType::Document(Document::Doc)),
            "docx" => Some(crate::MimeType::Document(Document::Docx)),
            "xls" => Some(crate::MimeType::Document(Document::Xls)),
            "xlsx" => Some(crate::MimeType::Document(Document::Xlsx)),
            "ppt" => Some(crate::MimeType::Document(Document::Ppt)),
            "pptx" => Some(crate::MimeType::Document(Document::Pptx)),
            "odt" => Some(crate::MimeType::Document(Document::Odt)),
            "ods" => Some(crate::MimeType::Document(Document::Ods)),
            "odp" => Some(crate::MimeType::Document(Document::Odp)),
            _ => None,
        }
    }

    fn from_mime(mime: &str) -> Option<crate::MimeType> {
        match mime {
            "application/msword" => Some(crate::MimeType::Document(Document::Doc)),
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document" => {
                Some(crate::MimeType::Document(Document::Docx))
            }
            "application/vnd.ms-excel" => Some(crate::MimeType::Document(Document::Xls)),
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet" => {
                Some(crate::MimeType::Document(Document::Xlsx))
            }
            "application/vnd.ms-powerpoint" => Some(crate::MimeType::Document(Document::Ppt)),
            "application/vnd.openxmlformats-officedocument.presentationml.presentation" => {
                Some(crate::MimeType::Document(Document::Pptx))
            }
            "application/vnd.oasis.opendocument.text" => {
                Some(crate::MimeType::Document(Document::Odt))
            }
            "application/vnd.oasis.opendocument.spreadsheet" => {
                Some(crate::MimeType::Document(Document::Ods))
            }
            "application/vnd.oasis.opendocument.presentation" => {
                Some(crate::MimeType::Document(Document::Odp))
            }
            _ => None,
        }
    }
}
