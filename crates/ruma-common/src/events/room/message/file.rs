use js_int::UInt;
use serde::{Deserialize, Serialize};

use crate::{
    events::room::{EncryptedFile, MediaSource, ThumbnailInfo},
    OwnedMxcUri,
};

/// The payload for a file message.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
#[serde(tag = "msgtype", rename = "m.file")]
pub struct FileMessageEventContent {
    /// A human-readable description of the file.
    ///
    /// This is recommended to be the filename of the original upload.
    pub body: String,

    /// The original filename of the uploaded file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,

    /// The source of the file.
    #[serde(flatten)]
    pub source: MediaSource,

    /// Metadata about the file referred to in `source`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Box<FileInfo>>,
}

impl FileMessageEventContent {
    /// Creates a new non-encrypted `FileMessageEventContent` with the given body, url and
    /// optional extra info.
    pub fn plain(body: String, url: OwnedMxcUri, info: Option<Box<FileInfo>>) -> Self {
        Self { body, filename: None, source: MediaSource::Plain(url), info }
    }

    /// Creates a new encrypted `FileMessageEventContent` with the given body and encrypted
    /// file.
    pub fn encrypted(body: String, file: EncryptedFile) -> Self {
        Self { body, filename: None, source: MediaSource::Encrypted(Box::new(file)), info: None }
    }
}

/// Metadata about a file.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
pub struct FileInfo {
    /// The mimetype of the file, e.g. "application/msword".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mimetype: Option<String>,

    /// The size of the file in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<UInt>,

    /// Metadata about the image referred to in `thumbnail_source`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_info: Option<Box<ThumbnailInfo>>,

    /// The source of the thumbnail of the file.
    #[serde(
        flatten,
        with = "crate::events::room::thumbnail_source_serde",
        skip_serializing_if = "Option::is_none"
    )]
    pub thumbnail_source: Option<MediaSource>,
}

impl FileInfo {
    /// Creates an empty `FileInfo`.
    pub fn new() -> Self {
        Self::default()
    }
}
