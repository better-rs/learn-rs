use fs_extra::{
    dir::{copy, copy_with_progress, create_all, CopyOptions, DirEntryAttr::Path},
    TransitProcess,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Error, Write},
    sync::{mpsc, mpsc::TryRecvError},
    thread, time,
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

    copy_dir();
    Ok(())
}

// fn copy_dir() {
//     let options = CopyOptions::new(); //Initialize default values for CopyOptions
//     // options.mirror_copy = true; // To mirror copy the whole structure of the source directory
//
//     let from_dir = "tmp/rs-template";
//     let to_dir = "tmp/rs-mid";
//
//     // copy source/dir1 to target/dir1
//     copy(from_dir, to_dir, &options)?;
// }

fn copy_dir() {
    let path_from = "tmp/rs-template";
    let path_to = "tmp/rs-mid";

    let mut options = CopyOptions::new();
    options.buffer_size = 1;
    options.content_only = true; // todo x: 只复制文件夹内容

    match create_all(&path_to, true) {
        Ok(_) => {},
        Err(err) => {
            println!("create dir failed, error: {}", err)
        },
    }

    let ret = copy(path_from, path_to, &options);
    match ret {
        Ok(_) => {
            println!("copy done!");
        },
        Err(err) => {
            println!("copy error: {}", err)
        },
    }
}
