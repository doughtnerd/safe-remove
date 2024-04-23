#![allow(dead_code, unused_variables)]
use base64::prelude::*;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub mod cache;

pub fn encode_file_path(file: &Path) -> Result<String, &'static str> {
    let file = file.canonicalize();

    if let Err(_) = file {
        return Err("Failed to encode file path to cacheable key");
    }
    let file = file.unwrap();

    let file = BASE64_STANDARD.encode(file.to_str().unwrap());

    Ok(file)
}

pub fn get_cache_dir() -> Result<PathBuf, String> {
    let current_dir = env::current_dir();

    if let Err(e) = current_dir {
        return Err(e.to_string());
    }

    let current_dir = current_dir.unwrap();

    let cache_dir = current_dir.join(".safe-remove-cache");

    if cache_dir.exists() {
        return Ok(cache_dir);
    }

    match fs::create_dir(&cache_dir) {
        Ok(_) => Ok(cache_dir),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_file_from_input() -> Result<PathBuf, String> {
    let filepath = std::env::args().skip(1).last();

    if let None = filepath {
        return Err("No file provided".to_string());
    }
    let filepath = filepath.unwrap();
    let file = Path::new(&filepath);

    if !file.exists() {
        let message = format!("File: {} does not exist", file.display());
        return Err(message);
    }

    Ok(file.to_owned())
}

pub fn copy_file_to_buffer(file: &Path) -> Result<Vec<u8>, String> {
    match fs::read(file) {
        Ok(data) => Ok(data),
        Err(e) => Err(e.to_string()),
    }
}

pub enum OS {
    Windows,
    MacOS,
    Linux,
    Other,
}

pub type CmdOptions = Vec<String>;

struct CmdOptionsS {
    force: bool,
    verbose: bool,
    dry_run: bool,
    remove: bool,
    restore: bool,
    list: bool,
    directory: bool,
    recursive: bool,
}

pub fn get_options() -> Option<CmdOptions> {
    let args = std::env::args();
    let args = args
        .skip(1)
        .rev()
        .skip(1)
        .collect::<Vec<String>>()
        .join("")
        .replace("-", "");

    if args.len() == 1 {
        return None;
    }

    None
}

pub fn get_variable_data_path() -> Result<String, String> {
    match get_os() {
        OS::Windows => Ok(String::from(
            "C:\\Users\\Username\\AppData\\Local\\SafeRemove",
        )),
        OS::MacOS => Ok(String::from("~/.cache/doughtnerd/safe_remove")),
        OS::Linux => Ok(String::from("~/.cache/doughtnerd/safe_remove")),
        _ => Err("Unsupported OS".to_string()),
    }
}

pub fn get_os() -> OS {
    if cfg!(target_os = "windows") {
        OS::Windows
    } else if cfg!(target_os = "macos") {
        OS::MacOS
    } else if cfg!(target_os = "linux") {
        OS::Linux
    } else {
        OS::Other
    }
}
