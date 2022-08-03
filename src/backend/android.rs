use crate::backend::DialogFutureType;
use crate::FileDialog;
use crate::FileHandle;
use std::path::PathBuf;

//
// File Picker
//

use crate::backend::FilePickerDialogImpl;
impl FilePickerDialogImpl for FileDialog {
    fn pick_file(self) -> Option<PathBuf> {
        todo!()
    }

    fn pick_files(self) -> Option<Vec<PathBuf>> {
        todo!()
    }
}

use std::pin::Pin;
use std::sync::{Arc, Mutex};

use std::future::Future;
use std::task::{Context, Poll, Waker};

struct Response {
    uri: String,
}
impl Response {
    fn from_jni(env: jni::JNIEnv, uri: jni::objects::JString) -> Option<Self> {
        let uri = env
            .get_string(uri)
            .ok()?
            .to_str()
            // TODO: I don't know if this is true.
            .expect("android.net.Uri.toString should give a valid UTF-8, rigth?")
            .to_string();

        Some(Self { uri })
    }
}

struct Callback {
    result: Arc<Mutex<Option<Response>>>,
    waker: Option<Waker>,
}
impl Callback {
    fn receive_response(&mut self, response: Option<Response>) {
        *self.result.lock().unwrap() = response;
        self.waker.take().unwrap().wake();
    }
}

struct FilePickFuture {
    folder: bool,
    result: Arc<Mutex<Option<Response>>>,
    called: bool,
}
impl FilePickFuture {
    fn new(folder: bool) -> Self {
        Self {
            folder,
            result: Arc::new(Mutex::new(None)),
            called: false,
        }
    }
}
impl Future for FilePickFuture {
    type Output = Option<FileHandle>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.called {
            false => {
                self.called = true;
                log::debug!("launch file picker!!");
                let android_context = ndk_context::android_context();
                let vm = std::sync::Arc::new(unsafe {
                    jni::JavaVM::from_raw(android_context.vm().cast()).unwrap()
                });
                jni::Executor::new(vm)
                    .with_attached(|env| {
                        let callback = Callback {
                            result: self.result.clone(),
                            waker: Some(cx.waker().clone()),
                        };
                        let callback = Box::leak(Box::new(callback));

                        let method = if self.folder {
                            "launchFolderPicker"
                        } else {
                            "launchFilePicker"
                        };

                        env.call_method(
                            android_context.context() as jni::sys::jobject,
                            method,
                            "(J)V",
                            &[jni::objects::JValue::Long(callback as *const _ as i64)],
                        )
                        .unwrap();
                        Ok(())
                    })
                    .unwrap();
                Poll::Pending
            }
            true => {
                let response = self.result.lock().unwrap().take();
                log::debug!("got uri {:?}", response.as_ref().map(|x| &x.uri));
                Poll::Ready(response.map(|x| FileHandle::wrap(x.uri)))
            }
        }
    }
}

/// Callback function for file picker result from Java.
///
/// Should be called by the JNI function, `native void filePickerResult(String uri, ByteBuffer data)`
#[allow(non_snake_case)]
pub extern "C" fn file_picker_result(
    env: jni::JNIEnv,
    class: jni::objects::JObject,
    callback_ptr: jni::sys::jlong,
    uri: jni::objects::JString,
) {
    // SAFETY: this callback_ptr was passed Java from rust, and was created with
    // Box::<Callback>>::leak().
    let mut callback = unsafe {
        let callback = callback_ptr as usize as *mut Callback;
        Box::from_raw(callback)
    };

    let response = Response::from_jni(env, uri);

    callback.receive_response(response);
}

use crate::backend::AsyncFilePickerDialogImpl;
impl AsyncFilePickerDialogImpl for FileDialog {
    fn pick_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        Box::pin(FilePickFuture::new(false))
    }

    fn pick_files_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        todo!()
    }
}

//
// Folder Picker
//

use crate::backend::FolderPickerDialogImpl;
impl FolderPickerDialogImpl for FileDialog {
    fn pick_folder(self) -> Option<PathBuf> {
        todo!()
    }

    fn pick_folders(self) -> Option<Vec<PathBuf>> {
        todo!()
    }
}

use crate::backend::AsyncFolderPickerDialogImpl;
impl AsyncFolderPickerDialogImpl for FileDialog {
    fn pick_folder_async(self) -> DialogFutureType<Option<FileHandle>> {
        Box::pin(FilePickFuture::new(true))
    }

    fn pick_folders_async(self) -> DialogFutureType<Option<Vec<FileHandle>>> {
        todo!()
    }
}

//
// File Save
//

use crate::backend::FileSaveDialogImpl;
impl FileSaveDialogImpl for FileDialog {
    fn save_file(self) -> Option<PathBuf> {
        todo!()
    }
}

use crate::backend::AsyncFileSaveDialogImpl;
impl AsyncFileSaveDialogImpl for FileDialog {
    fn save_file_async(self) -> DialogFutureType<Option<FileHandle>> {
        todo!()
    }
}
