use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

/// FileHandle is a way of abstracting over a file returned by a dialog
pub struct FileHandle{
    data: Vec<u8>,
}

impl FileHandle {
    pub fn wrap(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Get name of a file
    pub fn file_name(&self) -> String {
        todo!()
    }

    pub async fn read(&self) -> Vec<u8> {
        self.data.clone()
    }

    /// Unwraps a `FileHandle` and returns innet type.
    ///
    /// It should be used, if user wants to handle file read themselves
    ///
    /// On native platforms returns path.
    ///
    /// On `WASM32` it returns JS `File` object.
    ///
    /// #### Behind a `file-handle-inner` feature flag
    #[cfg(feature = "file-handle-inner")]
    pub fn inner(&self) -> &Path {
        &self.0
    }
}

impl std::fmt::Debug for FileHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileHandle(size {:?})", self.data.len())
    }
}
