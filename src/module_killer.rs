use std::env;
use std::fs;
use std::path::PathBuf;
use text_colorizer::*;

pub fn run() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let file_path = parse_args(args);
    let path = file_path.clone();
    let mut counter: u32 = 0;
    match kill_module(file_path, &mut counter) {
        Ok(_) => {
            eprintln!(
                "\u{1F977}  {} node_modules recursively removed  throughout  {:?}",
                counter, path
            )
        }
        Err(e) => {
            eprintln!("{}: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
    Ok(())
}

fn kill_module(path: PathBuf, count: &mut u32) -> Result<(), std::io::Error> {
    let metadata = fs::metadata(&path)?.file_type();
    if metadata.is_dir() {
        for entry in fs::read_dir(&path)? {
            let dir = entry?;
            if dir.file_name() == "node_modules" {
                fs::remove_dir_all(dir.path())?;
                *count += 1;
                eprintln!("removed {:?}", dir.path());
            } else {
                kill_module(dir.path(), count)?;
            }
        }
    }
    Ok(())
}

fn parse_args(args: Vec<String>) -> PathBuf {
    match args {
        args if args.is_empty() => {
            eprintln!(
                "{} Path argument is required ex:\n nodificator <path>\n",
                "Error:".red().bold()
            );
            std::process::exit(1)
        }
        args if args.len() > 1 => {
            eprintln!(
                "{} Nodificator require only one parameter path, ex: ./nodificator <path>",
                "Error:".red().bold()
            );
            std::process::exit(1)
        }
        args if args[0] == "--help" || args[0] == "h" => {
            call_help();
            std::process::exit(1)
        }
        _ => PathBuf::from(args[0].clone()),
    }
}

fn call_help() -> () {
    eprintln!("NODIFICATOR:\nA script to recursively remove all node_modules throughout a given folder.\nUsage: ./nodificator <path>\n   h, --help   This help text ")
}
