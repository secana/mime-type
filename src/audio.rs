use crate::MimeFormat;
use std::fmt::{self, Display, Formatter};

/// Audio file formats.
///
/// Supports common audio formats including lossy and lossless codecs.
pub enum Audio {
    /// MIDI audio format
    Midi,
    /// MPEG audio (MP3)
    Mpeg,
    /// MPEG-4 audio format
    M4a,
    /// OGG Vorbis audio format
    Ogg,
    /// FLAC lossless audio format
    Flac,
    /// WAV audio format
    Wav,
    /// AMR audio format
    Amr,
    /// AAC audio format
    Aac,
    /// AIFF audio format
    Aiff,
    /// DSF audio format
    Dsf,
    /// APE audio format
    Ape,
}

impl Display for Audio {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mime_str = match self {
            Audio::Midi => "audio/midi",
            Audio::Mpeg => "audio/mpeg",
            Audio::M4a => "audio/m4a",
            Audio::Ogg => "audio/ogg",
            Audio::Flac => "audio/x-flac",
            Audio::Wav => "audio/x-wav",
            Audio::Amr => "audio/amr",
            Audio::Aac => "audio/aac",
            Audio::Aiff => "audio/x-aiff",
            Audio::Dsf => "audio/x-dsf",
            Audio::Ape => "audio/x-ape",
        };
        write!(f, "{}", mime_str)
    }
}

impl MimeFormat for Audio {
    fn from_ext(ext: &str) -> Option<crate::MimeType> {
        match ext {
            "midi" | "mid" => Some(crate::MimeType::Audio(Audio::Midi)),
            "mp3" => Some(crate::MimeType::Audio(Audio::Mpeg)),
            "m4a" => Some(crate::MimeType::Audio(Audio::M4a)),
            "ogg" => Some(crate::MimeType::Audio(Audio::Ogg)),
            "flac" => Some(crate::MimeType::Audio(Audio::Flac)),
            "wav" => Some(crate::MimeType::Audio(Audio::Wav)),
            "amr" => Some(crate::MimeType::Audio(Audio::Amr)),
            "aac" => Some(crate::MimeType::Audio(Audio::Aac)),
            "aiff" | "aif" => Some(crate::MimeType::Audio(Audio::Aiff)),
            "dsf" => Some(crate::MimeType::Audio(Audio::Dsf)),
            "ape" => Some(crate::MimeType::Audio(Audio::Ape)),
            _ => None,
        }
    }

    fn from_mime(mime: &str) -> Option<crate::MimeType> {
        match mime {
            "audio/midi" => Some(crate::MimeType::Audio(Audio::Midi)),
            "audio/mpeg" => Some(crate::MimeType::Audio(Audio::Mpeg)),
            "audio/m4a" => Some(crate::MimeType::Audio(Audio::M4a)),
            "audio/ogg" => Some(crate::MimeType::Audio(Audio::Ogg)),
            "audio/x-flac" => Some(crate::MimeType::Audio(Audio::Flac)),
            "audio/x-wav" => Some(crate::MimeType::Audio(Audio::Wav)),
            "audio/amr" => Some(crate::MimeType::Audio(Audio::Amr)),
            "audio/aac" => Some(crate::MimeType::Audio(Audio::Aac)),
            "audio/x-aiff" => Some(crate::MimeType::Audio(Audio::Aiff)),
            "audio/x-dsf" => Some(crate::MimeType::Audio(Audio::Dsf)),
            "audio/x-ape" => Some(crate::MimeType::Audio(Audio::Ape)),
            _ => None,
        }
    }
}
