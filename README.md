![img](https://i.imgur.com/YPAgTdf.png)

[![version](https://img.shields.io/crates/v/rfd.svg)](https://crates.io/crates/rfd)
[![Documentation](https://docs.rs/rfd/badge.svg)](https://docs.rs/rfd)
[![dependency status](https://deps.rs/crate/rfd/0.8.4/status.svg)](https://deps.rs/crate/rfd/0.8.4)

Rusty File Dialogs is a cross platform Rust library for using native file open/save dialogs.
It provides both asynchronous and synchronous APIs. Supported platforms:

  * Windows
  * macOS
  * Linux & BSDs (GTK3 or XDG Desktop Portal)
  * WASM32 (async only)
  * Android (WIP)

Refer to the [documentation](https://docs.rs/rfd) for more details.


## Platform-specific notes

### Linux
Please refer to [Linux & BSD backends](https://docs.rs/rfd/latest/rfd/#linux--bsd-backends) for information about the needed dependencies to be able to compile on Linux.

### Android

The android implementation relies that the ndk context is a subclass of
NativeActivity, with the method `void launchFilePicker(long callback_ptr)`, and
that later calls `rfd::backend::android::file_picker_result` though jni, with
the given callback_ptr and the selected file Uri and data (null if canceled).

