use std::{
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
};

/// FileHandle is a way of abstracting over a file returned by a dialog
pub struct FileHandle {
    uri: String,
}

impl FileHandle {
    pub fn wrap(uri: String) -> Self {
        Self { uri }
    }

    /// Get name of a file
    pub fn file_name(&self) -> String {
        "Name".to_string()
    }

    pub async fn read(&self) -> Vec<u8> {
        let android_context = ndk_context::android_context();
        let vm = std::sync::Arc::new(unsafe {
            jni::JavaVM::from_raw(android_context.vm().cast()).unwrap()
        });
        jni::Executor::new(vm)
            .with_attached(|env| {
                let uri = env.new_string(self.uri.as_str())?;
                let buffer = env.call_method(
                    android_context.context() as jni::sys::jobject,
                    "readUri",
                    "(Ljava/lang/String;)Ljava/nio/ByteBuffer;",
                    &[uri.into()],
                )?;
                let buffer = match buffer {
                    jni::objects::JValue::Object(x) => jni::objects::JByteBuffer::from(x),
                    _ => return Err(jni::errors::Error::WrongJValueType("a", "b")),
                };

                let data = env.get_direct_buffer_address(buffer)?.to_vec();

                Ok(data)
            })
            .unwrap()
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
    pub fn inner(&self) -> &str {
        &self.uri
    }
}

impl std::fmt::Debug for FileHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FileHandle(size {:?})", self.uri)
    }
}
