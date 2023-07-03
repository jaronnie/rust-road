use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    // use method
    let f = read_file("test.txt".to_string());
    match f {
        Ok(content) => println!("Content: {}", String::from_utf8_lossy(&content)),
        Err(e) => println!("read file meet error: {}", e),
    }

    // 使用 ? 错误处理方式
    let f = read_file_v2("test.txt".to_string());
    match f {
        Ok(content) => println!("Content: {}", String::from_utf8_lossy(&content)),
        Err(e) => println!("read file meet error: {}", e),
    }
}

fn read_file(path: String) -> Result<Vec<u8>, io::Error> {
    let f = File::open(path);
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = Vec::new();
    match f.read_to_end(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_file_v2(path: String) -> Result<Vec<u8>, io::Error> {
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}
