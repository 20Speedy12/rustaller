use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FiletoPackage {
    installpath: String,
    path: String,
}

pub fn writetojson(path: &str, installpath: &str)
{
  println!("hi :D");
}
  
