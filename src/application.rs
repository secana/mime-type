use crate::{MimeFormat, MimeType};
use std::fmt::{self, Display, Formatter};

/// Application and executable file formats.
///
/// Supports various executable and binary application formats.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Application {
    /// WebAssembly binary format
    Wasm,
    /// Windows executable format
    Exe,
    /// Windows dynamic link library
    Dll,
    /// Executable and Linkable Format (Linux/Unix)
    Elf,
    /// LLVM bitcode format
    Bc,
    /// Mach-O binary format (macOS)
    Mach,
    /// Java class file
    Class,
    /// Dalvik executable format (Android)
    Dex,
    /// Optimized Dalvik executable (Android)
    Dey,
    /// X.509 certificate (DER encoded)
    Der,
    /// Object file format
    Obj,
    /// Javascript Object Notation
    Json
}

impl Display for Application {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mime_str = match self {
            Application::Wasm => "application/wasm",
            Application::Exe => "application/vnd.microsoft.portable-executable",
            Application::Dll => "application/vnd.microsoft.portable-executable",
            Application::Elf => "application/x-executable",
            Application::Bc => "application/llvm",
            Application::Mach => "application/x-mach-binary",
            Application::Class => "application/java",
            Application::Dex => "application/vnd.android.dex",
            Application::Dey => "application/vnd.android.dey",
            Application::Der => "application/x-x509-ca-cert",
            Application::Obj => "application/x-executable",
            Application::Json => "application/json",
        };
        write!(f, "{}", mime_str)
    }
}

impl MimeFormat for Application {
    fn from_ext(ext: &str) -> Option<MimeType> {
        match ext {
            "wasm" => Some(MimeType::Application(Application::Wasm)),
            "exe" => Some(MimeType::Application(Application::Exe)),
            "dll" => Some(MimeType::Application(Application::Dll)),
            "elf" => Some(MimeType::Application(Application::Elf)),
            "bc" => Some(MimeType::Application(Application::Bc)),
            "mach" => Some(MimeType::Application(Application::Mach)),
            "class" => Some(MimeType::Application(Application::Class)),
            "dex" => Some(MimeType::Application(Application::Dex)),
            "dey" => Some(MimeType::Application(Application::Dey)),
            "der" => Some(MimeType::Application(Application::Der)),
            "obj" => Some(MimeType::Application(Application::Obj)),
            "json" => Some(MimeType::Application(Application::Json)),
            _ => None,
        }
    }

    fn from_mime(mime: &str) -> Option<MimeType> {
        match mime {
            "application/wasm" => Some(MimeType::Application(Application::Wasm)),
            "application/vnd.microsoft.portable-executable" => {
                Some(MimeType::Application(Application::Exe))
            }
            "application/x-executable" => Some(MimeType::Application(Application::Elf)),
            "application/llvm" => Some(MimeType::Application(Application::Bc)),
            "application/x-mach-binary" => Some(MimeType::Application(Application::Mach)),
            "application/java" => Some(MimeType::Application(Application::Class)),
            "application/vnd.android.dex" => Some(MimeType::Application(Application::Dex)),
            "application/vnd.android.dey" => Some(MimeType::Application(Application::Dey)),
            "application/x-x509-ca-cert" => Some(MimeType::Application(Application::Der)),
            "application/json" => Some(MimeType::Application(Application::Json)),
            _ => None,
        }
    }
}
