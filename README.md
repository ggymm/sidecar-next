# sidecar-next

Rust + GPUI 简单示例，集成了 `gpui` 与 `gpui-component`，运行后会打开一个包含按钮的窗口。

## 依赖
- Rust 工具链（建议使用 rustup）
- 首次构建需联网拉取 git 依赖：
  - `gpui = { git = "https://github.com/zed-industries/zed.git" }`
  - `gpui-component = { git = "https://github.com/longbridge/gpui-component.git" }`

## 运行
- 开发运行：`cargo run`
- 发布构建：`cargo build --release`

运行后将出现一个窗口，居中显示 "Hello, GPUI!" 与一个可点击按钮。

## 目录
- `src/main.rs` 示例入口（基于 gpui + gpui-component）
- `Cargo.toml` 项目与依赖配置
- `.gitignore` 忽略构建产物与本地 vendor 目录
