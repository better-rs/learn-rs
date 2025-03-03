# rust wry

## Development

```ruby

cargo add wry

    
```


### Run

```ruby

cd git-repo-root-dir/

# multiwebview
task gui:wry:r -- --bin t01

# gtk_multiwebview
task gui:wry:r -- --bin t01b

# multiwindow
task gui:wry:r -- --bin t02

# streaming
task gui:wry:r -- --bin t03

# custom_titlebar, 自定义窗口标题栏 & 窗口关闭按扭
task gui:wry:r -- --bin t04

```



## Reference

- https://crates.io/crates/wry
- https://crates.io/crates/tao
- https://crates.io/crates/winit


> examples

- https://github.com/tauri-apps/wry/tree/dev/examples