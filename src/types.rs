use serde::{Serialize, Deserialize};

/// Data about a mime type
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MimeData {
    /// Common name of the mime type
    pub name: String,
    /// The mime type string
    pub mime: String,
    /// The corresponding file extension
    pub ext: String,
    /// Additional details about the mime type, often a spec that defined it
    pub details: String
}

/// Maps a mime type string to MimeData
pub type MimeDataMap = std::collections::HashMap<String, MimeData>;