use fs_extra::dir::{copy, create_all, CopyOptions};
use git2::Repository;
use rs_try::git::{git_clone, is_path_exist, rm_dir};
use std::path::Path;
use walkdir::WalkDir;

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

    let tpl_url = "https://github.com/better-rs/rs-template.git";
    let root_dir = "tmp";
    let tpl_dir = "tmp/rs-template";
    let mut tpl_sub_dir = "mono-repo/rs"; // 指定根路径
    let tmp_dir = "tmp/_new";
    let to_dir = "tmp/new-demo";

    let b = Builder::new(
        Some(root_dir.to_string()),
        tpl_url.to_string(),
        tpl_dir.to_string(),
        Some(tpl_sub_dir.to_string()),
        Some(tmp_dir.to_string()),
        to_dir.to_string(),
    );

    // 拉取模板:
    b.sync_template();

    // 新目录创建:
    b.create_repo();

    // 填入参数:
    b.render_repo();

    b.move_repo();

    b.print_info();
}

struct Builder {
    root_dir: String,
    tpl_url: String,
    tpl_dir: String,
    tpl_sub_dir: String,
    tmp_dir: String,
    to_dir: String,
}

impl Builder {
    pub fn new(
        root: Option<String>,
        tpl_url: String,
        tpl_dir: String,
        tpl_sub_dir: Option<String>,
        tmp_dir: Option<String>,
        to_dir: String,
    ) -> Self {
        Self {
            root_dir: root.unwrap_or("tmp".to_string()), // todo x: need change
            tpl_url,
            tpl_dir,
            tpl_sub_dir: tpl_sub_dir.unwrap_or("".to_string()),
            tmp_dir: tmp_dir.unwrap_or("_new".to_string()), // todo x: need change
            to_dir,
        }
    }

    pub fn parse_cli(&self) {}

    // 更新模板:
    pub fn sync_template(&self) {
        let path = &self.tpl_dir.as_ref();

        if is_path_exist(path) {
            println!("path is already exists, skip...");
            return
        }

        // clean:
        rm_dir(path);

        // git clone:
        git_clone(&self.tpl_url, path);
    }

    // 创建目录:
    pub fn create_repo(&self) {
        let mut options = CopyOptions::new();
        options.buffer_size = 1;
        options.content_only = true; // todo x: 只复制文件夹内容
        options.overwrite = true; // todo x: 覆盖式

        // 中间目录
        match create_all(&self.tmp_dir, true) {
            Ok(_) => {},
            Err(err) => {
                println!("create dir failed, error: {}", err)
            },
        }

        let p = Path::new(&self.tpl_dir).join(&self.tpl_sub_dir);

        println!("template path: {:?}", p.to_str());

        // todo x: 先创建临时目录
        let ret = copy(p, &self.tmp_dir, &options);
        match ret {
            Ok(_) => {
                println!("copy done!");
            },
            Err(err) => {
                println!("copy error: {}", err)
            },
        }
    }

    pub fn render_repo(&self) {
        // 遍历路径: 自动忽略无权限访问的路径
        for entry in WalkDir::new(&self.tmp_dir).into_iter().filter_map(|e| e.ok()) {
            println!("dir: {}", entry.path().display());
        }
    }

    pub fn move_repo(&self) {}

    pub fn print_info(&self) {
        println!("gent cli done!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_repo() {}
}
