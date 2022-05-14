use futures_util::AsyncReadExt;
use std::{
    path::PathBuf,
    process::{Command, Stdio},
    time::Duration,
};

use super::child_stdout::ChildStdout;
use crate::{file_dialog::Filter, FileDialog};

fn command() -> Command {
    Command::new("zenity")
}

fn add_filters(command: &mut Command, filters: &[Filter]) {
    for f in filters.iter() {
        command.arg("--file-filter");

        let extensions: Vec<_> = f
            .extensions
            .iter()
            .map(|ext| format!("*.{}", ext))
            .collect();

        command.arg(format!("{} | {}", f.name, extensions.join(" ")));
    }
}

fn add_filename(command: &mut Command, file_name: &Option<String>) {
    if let Some(name) = file_name.as_ref() {
        command.arg("--filename");
        command.arg(name);
    }
}

async fn run(mut command: Command) -> Option<String> {
    let mut process = command.stdout(Stdio::piped()).spawn().unwrap();

    dbg!(&process);

    let stdout = process.stdout.take().unwrap();
    let mut stdout = ChildStdout::new(stdout).unwrap();

    let mut buffer = String::new();
    stdout.read_to_string(&mut buffer).await.unwrap();

    let status = loop {
        if let Some(status) = process.try_wait().unwrap() {
            break status;
        }

        async_io::Timer::after(Duration::from_millis(1)).await;
    };

    if status.success() {
        Some(buffer)
    } else {
        None
    }
}

pub async fn pick_file(dialog: &FileDialog) -> Option<PathBuf> {
    let mut command = command();
    command.arg("--file-selection");

    add_filters(&mut command, &dialog.filters);
    add_filename(&mut command, &dialog.file_name);

    run(command).await.map(|buffer| {
        let trimed = buffer.trim();
        trimed.into()
    })
}

pub async fn pick_files(dialog: &FileDialog) -> Option<Vec<PathBuf>> {
    let mut command = command();
    command.args(&["--file-selection", "--multiple"]);

    add_filters(&mut command, &dialog.filters);
    add_filename(&mut command, &dialog.file_name);

    run(command).await.map(|buffer| {
        let list = buffer.trim().split("|").map(|s| PathBuf::from(s)).collect();
        list
    })
}

pub async fn pick_folder(dialog: &FileDialog) -> Option<PathBuf> {
    let mut command = command();
    command.args(&["--file-selection", "--directory"]);

    add_filters(&mut command, &dialog.filters);
    add_filename(&mut command, &dialog.file_name);

    run(command).await.map(|buffer| {
        let trimed = buffer.trim();
        trimed.into()
    })
}

pub async fn save_file(dialog: &FileDialog) -> Option<PathBuf> {
    let mut command = command();
    command.args(&["--file-selection", "--save", "--confirm-overwrite"]);

    add_filters(&mut command, &dialog.filters);
    add_filename(&mut command, &dialog.file_name);

    run(command).await.map(|buffer| {
        let trimed = buffer.trim();
        trimed.into()
    })
}

#[cfg(test)]
mod tests {
    use crate::FileDialog;

    #[test]
    fn pick_file() {
        let path = async_io::block_on(super::pick_file(&FileDialog::default()));
        dbg!(path);
    }

    #[test]
    fn pick_files() {
        let path = async_io::block_on(super::pick_files(&FileDialog::default()));
        dbg!(path);
    }

    #[test]
    fn pick_folder() {
        let path = async_io::block_on(super::pick_folder(&FileDialog::default()));
        dbg!(path);
    }

    #[test]
    fn save_file() {
        let path = async_io::block_on(super::save_file(&FileDialog::default()));
        dbg!(path);
    }
}
