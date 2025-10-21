# mime-type

A Rust library for working with MIME types and file extensions. This crate provides a simple, ergonomic API for converting between file extensions and their corresponding MIME type strings.

## Features

- Type-safe MIME type handling with enums for different categories
- Bidirectional conversion between file extensions and MIME type strings
- Multiple categories including images, videos, audio, documents, archives, fonts, books, and applications
- Zero dependencies and lightweight
- Display trait implementation for easy MIME type string output

## Usage

### Getting MIME type from file extension

```rust
use mime_type::{MimeType, MimeFormat};

// Get MIME type from extension
let mime = MimeType::from_ext("png");
assert_eq!(mime.unwrap().to_string(), "image/png");

// Works with multiple extensions for the same format
let jpeg1 = MimeType::from_ext("jpg");
let jpeg2 = MimeType::from_ext("jpeg");
assert_eq!(jpeg1.unwrap().to_string(), jpeg2.unwrap().to_string());

// Returns None for unknown extensions
let unknown = MimeType::from_ext("unknown");
assert!(unknown.is_none());
```

### Getting MIME type from MIME string

```rust
use mime_type::{MimeType, MimeFormat};

let mime = MimeType::from_mime("video/mp4");
match mime {
    Some(MimeType::Video(video)) => println!("It's a video: {}", video),
    _ => println!("Not a video"),
}
```

### Working with specific categories

```rust
use mime_type::{Image, Video, Audio, Document, MimeFormat};

// Directly use category enums
let png = Image::Png;
println!("{}", png); // Outputs: image/png

// Check extensions for specific categories
if let Some(mime) = Image::from_ext("webp") {
    println!("Found image: {}", mime);
}
```

### Pattern matching on MIME types

```rust
use mime_type::{MimeType, MimeFormat};

let mime = MimeType::from_ext("mp3").unwrap();

match mime {
    MimeType::Audio(audio) => println!("Audio file: {}", audio),
    MimeType::Video(video) => println!("Video file: {}", video),
    MimeType::Image(image) => println!("Image file: {}", image),
    _ => println!("Other file type"),
}
```

## Supported Categories

### Image Formats
JPEG, PNG, GIF, WebP, CR2, TIFF, BMP, HEIF, AVIF, JXR, PSD, ICO, ORA, DjVu

### Video Formats
MP4, M4V, MKV, WebM, MOV, AVI, WMV, MPG, FLV

### Audio Formats
MIDI, MP3, M4A, OGG, FLAC, WAV, AMR, AAC, AIFF, DSF, APE

### Document Formats
DOC, DOCX, XLS, XLSX, PPT, PPTX, ODT, ODS, ODP

### Archive Formats
ZIP, TAR, RAR, GZ, BZ2, 7Z, XZ, PDF, and many more

### Font Formats
TTF, OTF, WOFF, WOFF2

### Book Formats
EPUB, MOBI

### Application Formats
WASM, EXE, DLL, ELF, and various executable formats

## Edge Cases and Known Limitations

### Duplicate MIME Types

Some file formats share the same MIME type but have different extensions:

**EXE and DLL**: Both map to `application/vnd.microsoft.portable-executable`
- When converting from MIME type to extension, only `EXE` variant is returned
- This is by design as they share the same PE (Portable Executable) format

**ELF and OBJ**: Both map to `application/x-executable`
- When converting from MIME type, only `ELF` variant is returned
- These represent different executable object formats but share a MIME type

### EPUB Duplication

EPUB files appear in both `Archive` and `Book` categories:

- `Archive::Epub` maps to `application/epub+zip`
- `Book::Epub` also maps to `application/epub+zip`

**Behavior**:
- `MimeType::from_ext("epub")` returns `MimeType::Archive(Archive::Epub)` (Archive is checked first)
- `MimeType::from_mime("application/epub+zip")` returns `MimeType::Archive(Archive::Epub)`
- The `Book::Epub` variant is **effectively unreachable** through the standard API

**Recommendation**: Use `Archive::Epub` for EPUB files, or access `Book::Epub` directly if you need the Book variant specifically.

### Font MIME Type Ambiguity

Font formats have some MIME type overlap:

**TTF and OTF**: Both return `application/font-sfnt`
- When converting from this MIME type, only `TTF` is returned
- Both use the SFNT (Spline Font) container format

**WOFF and WOFF2**: Both currently return `application/font-woff`
- This is likely a bug; WOFF2 should use `font/woff2`
- When converting from MIME type, only `WOFF` is returned

### One-Way Mappings

Some conversions are not perfectly reversible:

```rust
// Extension -> MIME -> Extension may not round-trip
let mime = MimeType::from_ext("dll").unwrap();
// mime.to_string() == "application/vnd.microsoft.portable-executable"
// But from_mime() would return Application::Exe, not Dll
```

### Case Sensitivity

- File extensions are **case-sensitive** in the current implementation
- `.JPG` (uppercase) will not match, only `.jpg` (lowercase) will
- Always normalize extensions to lowercase before using this library

### Ambiguous File Types

Some formats could belong to multiple categories:

- **PDF**: Currently in `Archive` category, could arguably be in `Document`
- **RTF**: Currently in `Archive` category, but is a document format
- **SVG**: Not currently supported, could be `Image` or `Application`

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
