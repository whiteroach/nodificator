use std::env;
use text_colorizer::*;
mod module_killer;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    let file_path = parse_args(args);
    let mut counter = 0;
    match module_killer::kill_module(file_path.to_string(), &mut counter) {
        Ok(count) => {
            eprintln!(
                "\u{1F977}  {} node_modules recursively removed  throughout  {}",
                count, file_path
            )
        }
        Err(_) => {
            eprintln!("{}", "Error:".red().bold());
            std::process::exit(1);
        }
    }

    Ok(())
}

fn parse_args(args: Vec<String>) -> String {
    match args {
        args if args.is_empty() => {
            eprintln!(
                "{} path argument is required ex: cargo run <path/>",
                "Error:".red().bold()
            );
            std::process::exit(1)
        }
        args if args.len() > 1 => {
            eprintln!(
                "{} Nodificator require only one parameter path, ex: cargo run <path/>",
                "Error:".red().bold()
            );
            std::process::exit(1)
        }
        _ => args[0].clone(),
    }
}
