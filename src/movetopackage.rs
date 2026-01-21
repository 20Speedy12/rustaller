use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FiletoPackage {
    installpath: &str
    path: &str
}

pub fn writetojson(path: &str)
{
  println!("hi :D")
}
  
