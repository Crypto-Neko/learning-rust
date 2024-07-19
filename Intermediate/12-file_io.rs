use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::Result;

fn read_from_file(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn write_to_file(path: &str, contents: &str) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn main() -> Result<()> {
    let path = "data.txt";
    let contents = read_from_file(path)?;
    println!("File contents:\n{}", contents);
    
    let to_write = String::from("Written text.");
    write_to_file(path, &to_write)?;
    let new_contents = read_from_file(path);
    println!("New contents:\n{:?}", new_contents);
    Ok(())
}
