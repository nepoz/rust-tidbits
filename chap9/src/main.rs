use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let existing_file = read_file("hello.txt");
    let non_existing_file = read_file("nope.txt");

    match existing_file {
        Ok(s) => println!("Existing file says: {}", s),
        Err(_) => (),
    }

    match non_existing_file {
        Ok(_) => (),
        Err(e) => println!("Non-existent file says: {}", e)
    }
}

// Return file contents if exists, return error if not
fn read_file(filename: &str) -> Result<String, io::Error> {
    // Clean way of propogating erros; notice use of ?.
    let mut f = File::open(filename)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
