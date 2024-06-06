//! The manifest for all files contained in the zip.
//!
//! For unprocessed zip entries this also contains the actual bytes.

use get_size::GetSize;
use get_size_derive::GetSize;

/// A manifest entry.
#[derive(Debug, Clone, GetSize)]
pub struct Manifest {
    /// Path in the zip
    pub full_path: String,
    /// Version. Only used for the root entry "/".
    pub version: Option<String>,
    /// Mediatype.
    pub media_type: String,
    /// Unprocessed data is stored here.
    /// Everything except styles.xml, meta.xml, content.xml and settings.xml
    pub buffer: Option<Vec<u8>>,
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            full_path: "".to_string(),
            version: None,
            media_type: "".to_string(),
            buffer: None,
        }
    }
}

impl Manifest {
    /// Standard manifest entry without data.
    pub fn new<S: Into<String>, T: Into<String>>(full_path: S, media_type: T) -> Self {
        Self {
            full_path: full_path.into(),
            version: None,
            media_type: media_type.into(),
            buffer: None,
        }
    }

    /// Manifest entry with data.
    pub fn with_buf<S: Into<String>, T: Into<String>>(
        full_path: S,
        media_type: T,
        buf: Vec<u8>,
    ) -> Self {
        Self {
            full_path: full_path.into(),
            version: None,
            media_type: media_type.into(),
            buffer: Some(buf),
        }
    }

    /// Name ends with "/"
    pub fn is_dir(&self) -> bool {
        self.full_path.ends_with('/')
    }
}
