#[macro_use]
extern crate simple_error;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::path::Path;

use simple_error::SimpleError;

fn increname(name: &str, idx: i32) -> String {
    match idx {
        0 => name.to_string(),
        1 => name.to_string(),
        i => {
            let mut s = name.to_string();
            s.push_str(i.to_string().as_str());
            s
        }
    }
}

pub fn write_string(file_name: &str, text: &str) -> Result<usize, Box<dyn Error>> {
    let path = &Path::new(&file_name);
    let mut inf = File::create(path)?;
    Ok(inf.write(text.as_bytes())?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut a = std::env::args();

    a.next();

    let initfname = a.next().ok_or(SimpleError::new("blah"))?;

    let mut idx = 1;

    let mut fname = increname(initfname.as_str(), idx);

    while Path::exists(Path::new(&fname)) {
        idx = idx + 1;
        fname = increname(initfname.as_str(), idx);
    }

    let mut buffer = String::new();
    let mut stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_to_string(&mut buffer)?;

    write_string(fname.as_str(), buffer.as_str());

    Ok(())
}
