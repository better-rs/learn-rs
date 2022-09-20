// use rs_try::git::git_clone;

use git2::Repository;
use rs_try::{
    add,
    git::{git_clone, rm_dir},
};
use std::path::Path;
/*
    TODO X: 脚手架工具
        1. 命令行参数解析
        2. 全局配置目录创建 & 初始化
        3. 下载模板
        4. 基于模板, 创建新repo:
            a. 创建新项目, 先生成到 tmp/new-repo,
            b. 然后 render 此目录的模板文件.
            c. move/copy 到最终的路径下. 结束
        5. 兼容的模板工具:
            a. https://github.com/cookiecutter/cookiecutter

*/
fn main() {
    println!("hello gent!");

    add(1, 1);

    parse_cli();

    let url = "https://github.com/better-rs/rs-template.git";

    let path = "tmp/rs-template";

    // 拉取模板:
    sync_template(url, path.as_ref());

    new_repo();

    render_repo();

    move_repo();

    print_info();
}

fn parse_cli() {}

fn sync_template(url: &str, path: &Path) -> Repository {
    // clean:
    rm_dir(path);

    // git clone:
    git_clone(url, path)
}

fn new_repo() {}

fn render_repo() {}

fn move_repo() {}

fn print_info() {
    println!("gent cli done!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_repo() {}
}
