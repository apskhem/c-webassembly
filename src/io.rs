use std::fs;
use std::io::Write;
use std::path::Path;

pub fn read_file(path: &str) -> std::io::Result<String> {
    return fs::read_to_string(path);
}

pub fn write_file(path: &str, buf: &[u8]) -> std::io::Result<()> {
    let path = Path::new(path);

    if let Some(parent_path) = path.parent() {
        fs::create_dir_all(parent_path)?;
    }

    let mut out = fs::File::create(path)?;

    return out.write_all(buf);
}
