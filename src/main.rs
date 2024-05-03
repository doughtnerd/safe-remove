#![allow(unreachable_code, unused_variables, dead_code)]


use safe_remove::{cache_files, cli::{parse_main_cli_args, CliArgs}, find_matching_files, remove_files};

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
        CliArgs::RestoreArgs { dry, inputs } => {}
    }
}
