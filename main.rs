mod adb_utils;
use adb_utils::{ADBCommand, ADBManager};
use std::{
    collections::LinkedList,
    fs,
    path::{Path, PathBuf},
};

fn batch_rename(path: &str, mut start: i32) {
    let dir_content = fs::read_dir(path).unwrap();
    let mut file_names: LinkedList<String> = LinkedList::new();

    for entry in dir_content {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                if !entry.path().is_dir() {
                    file_names.push_back(file_name.to_owned());
                }
            }
        }
    }

    file_names.into_iter().for_each(|file| {
        if let Some(extension) = Path::new(&file).extension() {
            let old_path = PathBuf::from(format!("{}/{}", path, &file));
            let new_path = PathBuf::from(format!(
                "{}/{}.{}",
                path,
                start,
                extension.to_str().unwrap()
            ));

            if let Err(e) = fs::rename(old_path, new_path) {
                println!("error {}", e);
            } else {
                start += 1;
            }
        }
    });
}

fn main() {
    let mut adb = ADBManager::new();
    adb.connect("192.168.0.105", "46007").unwrap();
    adb.disconnect();
}
