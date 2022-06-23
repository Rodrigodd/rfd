//! FileHandle is a way of abstracting over a file returned by a dialog
//!
//! On native targets it just wraps a path of a file.
//! In web browsers it wraps `File` js object
//!
//! It should allow a user to treat web browser files same way as native files

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        mod web;
        pub use web::FileHandle;
    } else if #[cfg(target_os = "android")] {
        mod android;
        pub  use android::FileHandle;
    } else  {
        mod native;
        pub use native::FileHandle;
    }
}

#[cfg(test)]
mod tests {
    use super::FileHandle;

    #[test]
    fn fn_def_check() {
        let _ = FileHandle::wrap;
        let _ = FileHandle::read;
        #[cfg(feature = "file-handle-inner")]
        let _ = FileHandle::inner;
        #[cfg(not(target_arch = "wasm32"))]
        let _ = FileHandle::path;
    }
}
