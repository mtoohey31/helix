use serde::{Deserialize, Serialize};

use crate::{OptionalVersionedTextDocumentIdentifier, Range};

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum FileProgressKind {
    Processing,
    FatalError,
}

impl<'de> Deserialize<'de> for FileProgressKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match <u8 as Deserialize>::deserialize(deserializer)? {
            1 => Ok(FileProgressKind::Processing),
            2 => Ok(FileProgressKind::FatalError),
            other => Err(serde::de::Error::custom(format_args!(
                "invalid value: {other}, expected 1 or 2"
            ))),
        }
    }
}

impl Serialize for FileProgressKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            FileProgressKind::Processing => 1,
            FileProgressKind::FatalError => 2,
        };
        Serialize::serialize(&value, serializer)
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileProgressProcessingInfo {
    pub range: Range,
    pub kind: FileProgressKind,
}

/// The file progress notification is sent from the server to the client to
/// inform it of how much of the file has been processed.
#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileProgressParams {
    /// The document whose progress changed.
    pub text_document: OptionalVersionedTextDocumentIdentifier,

    /// The ranges of the file that are currently being processed by the server.
    pub processing: Vec<FileProgressProcessingInfo>,
}
