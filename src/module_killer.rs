use std::fs;
use std::path::PathBuf;

pub fn kill_module(path: PathBuf, count: &mut u32) -> Result<(), std::io::Error> {
    let metadata = fs::metadata(&path)?.file_type();
    if metadata.is_dir() {
        for entry in fs::read_dir(&path)? {
            let dir = entry?;
            if dir.file_name() == "node_modules" {
                // fs::remove_dir_all(dir.path())?;
                *count += 1;
                eprintln!("removed {:?}", dir.path());
            } else {
                kill_module(dir.path(), count)?;
            }
        }
    }
    Ok(())
}
