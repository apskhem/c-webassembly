use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

pub struct ArgumentCollection {
    in_file: Option<String>,
    out_file: Option<String>,
}

impl ArgumentCollection {
    fn new(in_file: Option<String>, out_file: Option<String>) -> ArgumentCollection {
        return ArgumentCollection { in_file, out_file };
    }
}

pub fn parse_args(args: env::Args) -> Result<ArgumentCollection, &'static str> {
    let args: Vec<String> = env::args().collect();
    let in_file = match args.get(1) {
        Some(x) => x,
        None => return Err("File path to be compiled is missing."),
    };

    let coll = ArgumentCollection::new(Some(in_file.clone()), None);

    return Ok(coll);
}

pub fn read_file(path: &str) -> std::io::Result<String> {
    return fs::read_to_string(path);
}

pub fn write_file(path: &str) -> std::io::Result<()> {
    let path = Path::new(path);

    if let Some(parent_path) = path.parent() {
        fs::create_dir_all(parent_path)?;
    }

    let a: Vec<u8> = vec![
        0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    ];

    let mut out = fs::File::create(path)?;

    return out.write_all(&a);
}
