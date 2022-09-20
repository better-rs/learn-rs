# rust-try:

> 用途:

- 💡 调研各种社区 lib, 需要写 demo 验证.
- 💡 此 repo, 用于此目的.

> 用例代码入口:

- ✅ [bin/](bin)

> 启动脚本:

- ✅ [Taskfile.yml](Taskfile.yml)
- ✅ 执行如下启动命令, 运行示例代码

```ruby
cd learn-rs/

task try:run:template

task try:run:git

task try:run:dir

task try:run:file

```

## include:

> rust git 包:

- ✅ 用例: [git.rs](bin/git.rs)
- ✅ https://docs.rs/git2/latest/git2/

> rust template 模板包:

- ✅ 用例: [template.rs](bin/template.rs)
- ✅ https://github.com/djc/askama
- ✅ 参考示例:
    - https://github.com/tyrchen/geektime-rust/blob/master/47_48_macros/src/raw_builder.rs#L8
-

> rust 目录遍历:

- ✅ 用例: [dir.rs](bin/dir.rs)
- ✅ https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
- ✅ https://crates.io/crates/walkdir
    - https://github.com/BurntSushi/walkdir
- ✅ https://github.com/cgag/loc

> rust 文件读写:

- ✅ 用例: [file.rs](bin/file.rs)
- ✅ https://rust-lang-nursery.github.io/rust-cookbook/file/read-write.html
