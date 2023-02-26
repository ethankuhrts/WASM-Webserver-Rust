use std::{
    fs, path::{Path, PathBuf}, io::Read,
};
use crate::Error;

pub struct Templates {}

impl Templates {
    
    pub fn render(file_path: &str) -> Result<String, Error> {
        let path = fs::canonicalize(PathBuf::from(format!("res/{file_path}"))).unwrap();
        match fs::read_to_string(path) {
            Ok(res) => Ok(res),
            Err(err) => {println!("{err}"); Err(Error::FileNotFound)}
        }
    }

    pub fn render_bytes(file_path: &str) -> Result<Vec<u8>, Error> {
        let path = fs::canonicalize(PathBuf::from(format!("res/{file_path}"))).unwrap();

        let mut f = match fs::File::open(path) {
            Ok(res) => res,
            Err(err) => return Err(Error::FileNotFound),
        };
        
        let mut buffer = vec![0; f.metadata().unwrap().len() as usize];
        let n = f.read(&mut buffer[..]);
        return Ok(buffer);
    }

}