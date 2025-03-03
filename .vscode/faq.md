# vscode faq



## rust + rust-analyzer

- 默认子项目如果不在根目录的 `workspace` 内, vscode 需要添加自定义的路径到 `settings.json` 中.
- https://rustcc.cn/article?id=02cf99e7-9de3-44ef-b6d4-97720ce1f00c


```json

    "rust-analyzer.linkedProjects": [
        "path/to/Cargo.toml"
    ]
```