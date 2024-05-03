#![allow(dead_code, unused_variables)]
use base64::prelude::*;
use cache::{DiskCache, Cache};
use glob::glob;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub mod cache;
pub mod cli;

pub fn remove_files(files: &Vec<PathBuf>) {
    for file in files {
        fs::remove_file(file).unwrap_or_default();
    }
}

pub fn find_matching_files(filename_patterns: Vec<String>, include_dirs: bool) -> Vec<PathBuf> {
    filename_patterns
        .into_iter()
        .filter(|file| {
            if file == ".." || file == "." {
                return false;
            }
            return true;
        })
        .flat_map(|file| {
            glob(file.as_str()).unwrap()
        })
        .flatten()
        .filter(|file| {
            if file.is_dir() {
                return include_dirs;
            }
            return true;
        })
        .map(|file| file.canonicalize().unwrap())
        .collect()
}

pub fn cache_files(files: &Vec<PathBuf>) {
    let cache_dir = get_cache_dir();
    if let Err(e) = cache_dir {
        eprintln!("Failed to get cache dir\r\n{}", e);
        return;
    }

    let cache = DiskCache::new(cache_dir.unwrap());
    let mut cache = match cache {
        Ok(cache) => cache,
        Err(e) => {
            eprintln!("Failed to create cache {}", e);
            return;
        }
    };

    for file in files {
        let data = copy_file_to_buffer(&file);
        let data = match data {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Failed to copy file data:\r\n{}", e);
                return;
            }
        };

        let encoded_path = encode_file_path(&file);
        if let Err(e) = encoded_path {
            eprintln!("Failed to encode file path\r\n{}", e);
            return;
        }
        let encoded_path = encoded_path.unwrap();
        
        cache.store(&encoded_path, data);

        println!("File {} has been safely removed", file.display());
    }
}

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
