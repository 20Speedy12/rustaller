use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FiletoPackage {
    installpath: String,
    file: String,
}

pub fn writetojson(paths: &str, installpaths: &str)
{
    let qwa = "./bin/";
    let filename = Path::new(paths).file_name().unwrap().to_str().unwrap();
    let internalfilepath = qwa.to_owned() + filename;
    fs::copy(paths, internalfilepath);
    let mut filer = FiletoPackage {
    installpath: installpaths.to_owned(),
    file: filename.to_owned()

}
  
