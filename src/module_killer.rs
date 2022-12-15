use std::fs;
use text_colorizer::*;

pub fn kill_module(file_path: String) -> Result<u32, std::io::Error> {
    static mut COUNTER: u32 = 0;
    let metadata = fs::metadata(&file_path)?.file_type();
    if metadata.is_dir() {
        for entry in fs::read_dir(&file_path)? {
            let dir = entry?;
            if dir.file_name() == "node_modules" {
                unsafe {
                    COUNTER += 1;
                }
                eprintln!(
                    "removed {}/node_modules",
                    &file_path.green().bold(),
                );

                // match fs::remove_dir_all(file_path.to_string() + "/node_modules") {
                //     Ok(_) =>  eprintln!("removed"),
                //     Err(e) => eprintln!("{} cannot remove: {e}", "Error:".red().bold())
                // }//map the error
            } else {
                if let Some(file_name) = dir.file_name().to_str() {
                    match kill_module(file_path.to_string() + "/" + file_name) {
                        Ok(counter) => counter,
                        Err(e) => {
                            eprintln!("{} cannot remove: {e}", "Error:".red().bold());
                            std::process::exit(1)
                        }
                    };
                }
            }
        }
    }
    unsafe { Ok(COUNTER) }
}
