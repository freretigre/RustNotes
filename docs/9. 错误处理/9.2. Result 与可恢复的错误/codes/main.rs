use std::fs::File;
use std::io::{self, Read};

// panic! 与不可恢复的错误
fn main() {
    
}

fn read_username_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(e) => return Err(e);
  };

  let mut s = String::new();
  
  match f.read_to_string(&mut s) {
    OK(_) => Ok(s),
    Err(e) => Err(e),
  }
}

