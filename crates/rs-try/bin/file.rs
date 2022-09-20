use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Write},
};

/*
    TODO X:
        1. rust 基本的文件读写用法.
        2. 参考: https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html

*/
fn main() -> Result<(), Error> {
    // 文件路径:
    let path = "tmp/lines.txt";

    // todo x: 写文件
    let mut output = File::create(path)?;
    write!(output, "Rust\n💖\nFun")?;

    // todo x: 读文件
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("read: {}", line?);
    }

    Ok(())
}
