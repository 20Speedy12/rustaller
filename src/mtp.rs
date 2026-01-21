use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FiletoPackage {
    installpath: &str,
    file: &str,
}

pub fn writetojson(paths: &str, installpaths: &str)
{
    let filename = Path::new(paths).file_name().unwrap().to_str().unwrap();
    fs::copy(paths, "./bin/{}", filename)
  let mut filer = FiletoPackage {
    installpath: installpaths.to_owned
    file: filename.to_owned
  }
}
  
