#![allow(
    unreachable_code,
    unused_variables,
    dead_code,
    unused_parens,
    unused_imports
)]
use clap::{
    builder::{PathBufValueParser, StringValueParser},
    Arg, ArgAction, Command,
};
use glob::{glob, glob_with, MatchOptions, Paths};
use globset::{Glob, GlobSetBuilder};
use safe_remove::{
    cache::{Cache, DiskCache},
    copy_file_to_buffer, encode_file_path, get_cache_dir, get_file_from_input, get_options,
};
use std::{
    env::{self, current_dir},
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

fn main() {
    let matches = Command::new("Safe Remove")
        .author("doughtnerd@github.com")
        .version("0.1.0")
        .about("Remove files with the option to restore them")
        .arg(
            Arg::new("recursive")
                .short('r')
                .long("recursive")
                .action(ArgAction::SetTrue)
                .help("Recursively remove files"),
        )
        .arg(
            Arg::new("directory")
                .short('d')
                .long("directory")
                .help("Remove directories")
                .action(ArgAction::SetTrue)
        )
        .arg(
            Arg::new("file")
                .action(ArgAction::Append)
                .required(true)
                .help("The file(s) or glob pattern(s) to remove"),
        )
        .get_matches();

    let mut files = Vec::new();
    matches
        .get_many::<String>("file")
        .unwrap_or_default()
        .into_iter()
        .for_each(|file| {
            if let Ok(paths) = glob(file) {
                files.extend(paths);
            }
        });

    let files = files.into_iter().flatten().filter(|file| {
        if file.is_dir() {
            let num_files = fs::read_dir(file).unwrap().into_iter().count();
            if matches.get_flag("recursive") {
                return num_files > 0;
            }
            return false;
        }
        return true;
    });

    for file in files {
        println!("Glob Match: {:?}", file);
    }
    return;

    let options = get_options();

    let file = get_file_from_input();
    let file = match file {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let data = copy_file_to_buffer(&file);
    let data = match data {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to copy file data {}", e);
            return;
        }
    };

    let cache_dir = get_cache_dir();
    if let Err(e) = cache_dir {
        println!("Failed to get cache dir {}", e);
        return;
    }

    let encoded_path = encode_file_path(&file);
    if let Err(e) = encoded_path {
        println!("Failed to encode file path {}", e);
        return;
    }
    let encoded_path = encoded_path.unwrap();

    let cache = DiskCache::new(cache_dir.unwrap());
    let mut cache = match cache {
        Ok(cache) => cache,
        Err(e) => {
            println!("Failed to create cache {}", e);
            return;
        }
    };

    cache.store(&encoded_path, data);

    println!("File {} has been safely removed", file.display());
}
