# Tauri + Vue3 + Naive-UI Template

> 特性:

- ✅ 激活`系统托盘(tray)`图标(MacOS).
- ✅ `ts/js` 和 `rust` 互操作代码示例(js call rust).
- ✅ 前端框架: `typescript + vue3 + naive-ui`.

> 运行效果图:

- 主窗口

![Screenshot](./public/screenshot.png)

- 系统托盘图标:

![Screenshot](./public/screenshot2.png)

## Rust 插件集成:

- https://github.com/tauri-apps/awesome-tauri#plugins

> 插件集成列表:

- ✅ 集成多语言插件(i18n): [rust-i18n](https://github.com/longbridgeapp/rust-i18n)
- ✅ 集成日志模块: tracing
    - [ ] [tauri-plugin-log](https://github.com/tauri-apps/tauri-plugin-log)
- ✅ 集成 db: sqlx + sqlite:
    - [tauri-plugin-sql](https://github.com/tauri-apps/tauri-plugin-sql)
- ✅ 集成 单例模式(app context), 全局变量:
    - [once_cell](https://github.com/matklad/once_cell)
- ✅ 集成 kv db 存储:
    - 加密 kv 存储: [microkv](https://github.com/ex0dus-0x/microkv)
    - kv 存储:
        - [sled](https://github.com/spacejam/sled)
        - [rust-kv](https://github.com/zshipko/rust-kv)
        - [pickledb](https://crates.io/crates/pickledb)
- [ ] 集成 内存型 cache kv 方案:
    - [microkv](https://github.com/ex0dus-0x/microkv)
    - [redb](https://github.com/cberner/redb)
    - [nebari](https://github.com/khonsulabs/nebari)
    - [memdb](https://github.com/yoshuawuyts/memdb)
- [ ] 集成 cache:
    - [cached](https://lib.rs/crates/cached)
    - [lru-rs](https://github.com/jeromefroe/lru-rs)
    - [moka](https://github.com/moka-rs/moka)
- [ ] 集成 cronjob 定时任务:
    - [cron](https://crates.io/crates/cron)
    - [tokio-cron-scheduler](https://crates.io/crates/tokio-cron-scheduler)
- [ ] 集成 retry 重试:
    - [tokio-retry](https://github.com/srijs/rust-tokio-retry)
    - [again](https://crates.io/crates/again)
- [ ] 集成 websocket 插件:
    - [tauri-plugin-websocket](https://github.com/tauri-apps/tauri-plugin-websocket)
    - [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite)
        - 当前社区首选

## Rust 踩坑:

> 🙅 修复 tray icon 显示黑块问题:

- ❌ `iconAsTemplate` 请设置为 `false`.

```ruby


       "systemTray": {
            "iconPath": "icons/icon.png",
            "iconAsTemplate": false // 注意: 改为 false 即可
        },

```

> 🙅 cargo workspace 问题:

- ❌ `子 repo`, 尽量独立, 不要使用`父 workspace`, 会导致 tauri build 失败
    - case: sqlx 在 workspace 下, build [依赖报错(误报)](https://github.com/launchbadge/sqlx/issues/1604), 但是单独
      build 是可以的
- ❌ workspace 过大, 会导致 IDE 索引宏, 失效
- ❌ 猜测: workspace 公共的lib, 版本(开启 feature)不一致, 在编译时, 会冲突.
    - 部分冲突, 是隐式的. cargo clean + build 就会发现很多问题

> 🙅 sqlx 问题:

- ❌ `sqlx::query!() vs sqlx::query()`
    - query!() 宏误报 `找不到 db`, 而 query() 方法写法不会误报
    - new() 方法中含有 db conn 初始化
    - 猜测 query!() 是编译期, 尝试找 db conn, 而 query() 是运行时, 所以不会报错.
- ❌ 故不建议在 app + sqlite 代码中使用 query!() 宏,
    - 搜索 GitHub 代码, 发现大多数人, 也都不用 query!().
      [    - 官方 example 给的示例代码, 误导人.
    - query() 方法, 虽然用起来, 稍微麻烦一点, 但是运行时检测 db conn, 是更安全的.
    - 在运行时动态创建 sqlite 是很正常的.
    - 当然, 如果是写 sever 代码(远程连 mysql 这种), 继续使用 query!() 宏, 应该没问题(db一般都预创建好了)

## Vue.js 集成:

- [package.json](package.json)
- ✅ naive-ui: Vue3 UI 组件库
- ✅ vue-router
- ✅ vite
- ✅ typescript

![Screenshot](./public/screenshot.png)


---

Simple project template for Tauri and Vue 3.

## Features

- Vue 3 / TypeScript frontend
    - [ESLint](https://eslint.org/) + [Prettier](https://prettier.io/) configured
    - [Vue-devtools](https://devtools.vuejs.org/) installed
    - [Tailwind CSS](https://tailwindcss.com/) w/ PostCSS configured
- Simple and fast [Vite](https://vitejs.dev/) config w/ HMR in development and optimizations for production builds
    - [AutoImport plugin](https://github.com/antfu/unplugin-auto-import)
- [Vitest](https://vitest.dev/) for unit tests
- Github Actions for testing and building
- Debugging configuration set up for VS Code

## Setting Up

1. Install [Tauri Prerequisites](https://tauri.studio/v1/guides/getting-started/prerequisites)
2. Clone and install dependencies (this template uses `pnpm` by default):

```sh
pnpm i
```

## Usage

A Tauri app has at least [two processes](https://tauri.app/v1/guides/architecture/process-model):

- the Core Process (`backend`, or _main_ process in Electron terminology), and
- the WebView process (`frontend`, or _renderer_ in Electron)

### 🦢 Frontend (TS, PnPM)

#### Running Development Server

Both back- and frontend start with a single command:

```sh
pnpm dev
```

#### Testing

```sh
pnpm test
```

### 🦀 Backend (Rust, Cargo)

Backend code lives in `src-tauri/` (Following commands are to be run from there.)

#### Finding Outdated Rust Dependencies

If you have [cargo-outdated](https://github.com/kbknapp/cargo-outdated) installed:

```sh
cargo outdated
```

#### Upgrading Rust Dependencies

If you have [cargo-edit](https://github.com/killercup/cargo-edit) installed:

```sh
cargo upgrade
```

### Debugging

- The `dev` command has by default `RUST_BACKTRACE=1` set which makes Rust output full backtraces to the console. (
  Simply remove it from the package.json command if you want it).
- If you use VS Code, you can debug Rust code with the included `Debug Tauri` config.

### Building and releasing

#### Building

The project has GitHub Actions set up which will automatically test and build your app with every push and PR. For
building manually:

```sh
pnpm build
```

#### Releasing a new version

1. Bump version number (In `package.json`, and `src-tauri/tauri.conf.json`)
2. Run `pnpm check` to update `Cargo.lock`
3. Tag the commit you want to release with `vX.Y.Z`
4. Edit the release notes and push (also tags!)
5. Github workflow will automatically build a new draft release for this version. Publish when ready 🎉

## Howto

### Custom title bar styles (like `titleBarStyle: 'hidden'` in Electron)

Tauri doesn't currently offer a method to hide the title bar without hiding all window chrome. There is, however, a
fairly simple way to do it manually (with certain limitations;
see [Tauri issue #2663](https://github.com/tauri-apps/tauri/issues/2663) for details).

1. Add `cocoa` and `objc` crates to dependencies
2. Add `set_transparent_titlebar` and `position_traffic_lights` from this
   gist: https://gist.github.com/Uninen/5377381eb81bdcd71b9d1859e46e3e29
3. Call `set_transparent_titlebar` and `position_traffic_lights` from `setup` and `on_window_event` (example in the gist
   starting line 114) on any window you want affected.

This implementation works but results in visible jerkyness of the traffic lights (on macOS) when the window is
resized. (Alternatives discussed in detail in the issue #2663)

## Elsewhere

- [Follow @uninen](https://twitter.com/uninen) on Twitter
- Read my continuously updating learnings around Tauri / Vue / TypeScript and other Web development topics from
  my [Today I Learned site](https://til.unessa.net/)

## Contributing

Contributions are welcome! Please follow the [code of conduct](./CODE_OF_CONDUCT.md) when interacting with others.
