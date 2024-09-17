use std::path::PathBuf;

#[derive(Clone)]
pub struct Args {
    pub filepath: PathBuf,
    pub directory: String,
    pub filename: String,
    pub extension: String,
}

impl Args {
    pub fn new() -> Result<Self, &'static str> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() != 2 {
            return Err("Usage: <input_file>");
        }

        let filepath = PathBuf::from(&args[1]);
        let directory = filepath.parent().unwrap().to_str().unwrap().to_string();
        let filename = filepath.file_stem().unwrap().to_str().unwrap().to_string();
        let extension = filepath.extension().unwrap_or_default().to_str().unwrap_or_default().to_string();

        Ok(Self {
            filepath,
            directory,
            filename,
            extension,
        })
    }
}