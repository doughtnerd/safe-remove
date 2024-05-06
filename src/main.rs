#![allow(unreachable_code, unused_variables, dead_code)]


use std::{fs, path::PathBuf};

use safe_remove::{cache_files, cli::{parse_main_cli_args, CliArgs}, encode_file_path, find_matching_files, remove_files, restore_files};

fn main() {
    let args = match parse_main_cli_args() {
        Ok(args) => args,
        Err(msg) => {
            panic!("")
        }
    };

    match args {
        CliArgs::RemoveArgs { cache, recursive, directory, dry, files } => {
            let files = find_matching_files(files, directory);
            
            if dry {
                println!("{:#?}", files);
                return;
            }

            if cache {
                cache_files(&files);
            }

            remove_files(&files);
        },
        CliArgs::RestoreArgs { dry, files: inputs } => {
            let canonical_paths = convert_to_canonical(&inputs);

            println!("{:?}", inputs);

            let hash_paths: Vec<(PathBuf, String)> = convert_to_hashes(canonical_paths);
            println!("{:?}", hash_paths);
            let hashes = hash_paths
                .into_iter()
                .map(|p| {
                    p.1
                })
                .collect();
            restore_files(&hashes)
        }
    }
}

fn convert_to_canonical(inputs: &Vec<String>) -> Vec<PathBuf> {
    inputs
        .into_iter()
        .map(|file| PathBuf::from(file))
        .map(|file| fs::canonicalize(".").unwrap().join(PathBuf::from(file)))
        .collect()
}

fn convert_to_hashes(inputs: Vec<PathBuf>) -> Vec<(PathBuf, String)> {
    inputs
        .into_iter()
        .map(|file| {
            let encoded = encode_file_path(&file).unwrap();
            println!("{:?}/{:?}", encoded, file);
            (file, encoded)
        })
        .collect()
}
