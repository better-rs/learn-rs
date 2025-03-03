use walkdir::WalkDir;

/*
    TODO X:
        1. 目录遍历
        2. 参考: https://github.com/BurntSushi/walkdir

*/
fn main() {
    // 目标路径:
    let dir = "src";

    // 遍历路径: 自动忽略无权限访问的路径
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        println!("dir: {}", entry.path().display());
    }
}
