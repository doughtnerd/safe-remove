use clap::{arg, ArgAction, Command};

#[derive(Debug)]
pub enum CliArgs {
    RemoveArgs {
        recursive: bool,
        directory: bool,
        dry: bool,
        files: Vec<String>
    },
    RestoreArgs {
        dry: bool,
        inputs: Vec<String>
    }
}

/// Runs the main CLI argument parser to get the arguments passed to the program.
pub fn parse_main_cli_args() -> Result<CliArgs, &'static str> {
    let parsed_args = Command::new("Safe Remove")
        .author("doughtnerd@github.com")
        .version("1.0.0")
        .about("Remove files with the option to restore them")
        .subcommands([
            Command::new("remove")
                .about("Remove files with the option to restore them")
                .args([
                    arg!(dry: -D --dry "Show the files to be removed without actually removing them"),
                    arg!(recursive: -r --recursive "Recursively remove files").action(ArgAction::SetTrue),
                    arg!(directory: -d --directory "Remove directories").action(ArgAction::SetTrue),
                    arg!(file: ... "The file(s) or glob pattern(s) to remove or restore").required(true),
                ]),
            Command::new("restore")
                .about("Restore files that have been removed")
                .args([
                    arg!(dry: -D --dry "Show the files to be restored without actually restoring them"),
                    arg!(file: ... "The file(s) or glob pattern(s) to restore").required(true),
                ])
        ])
        .get_matches();

    match parsed_args.subcommand() {
        Some(("remove", args)) => {
            Ok(
                CliArgs::RemoveArgs {
                    directory: args.get_flag("directory"),
                    dry: args.get_flag("dry"),
                    recursive: args.get_flag("recursive"),
                    files: args
                        .get_many::<String>("file")
                        .unwrap()
                        .map(|file| file.to_string())
                        .collect()
                }
            )
        },
        Some(("restore", args)) => {
            Ok(
                CliArgs::RestoreArgs {
                    dry: args.get_flag("dry"),
                    inputs: args
                        .get_many::<String>("inputs")
                        .unwrap()
                        .map(|input|  input.to_string())
                        .collect()
                }
            )
        },
        _ => Err("Unrecognized arguments")
    }
}