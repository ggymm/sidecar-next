1. 将当前项目使用到的gpui-component组件（~/.cargo/git/checkouts/gpui-component）
2. 复制或者在src/comps中重新实现，实现自定义
3. 需要注意使用当前最新版本的gpui代码
4. 首先将项目中使用到的组件全部罗列出来
5. 按照复杂顺序，依次实现替换，最后移除gpui-component依赖


参考 gpui-component组件（~/.cargo/git/checkouts/gpui-component-95ce574d8a0da8b8）
和 https://raw.githubusercontent.com/zed-industries/zed/refs/heads/main/crates/gpui/examples/input.rs
实现输入组件，替换当前项目中的实现