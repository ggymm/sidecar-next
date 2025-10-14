# 项目地图（sidecar-next）

## 核心结构
- `src/main.rs`：gpui 桌面应用入口，初始化主题、窗口与 `MainView`。`MainView` 负责侧边栏导航、内容区动态切换，并缓存所有页面对应的 `AnyView`。
- `src/comps/mod.rs`：统一的 UI 组件皮肤（page/card/label/input/button/textarea），封装了项目的暗色主题和常见布局。
- `assets/`：侧边栏图标与 LOGO，路径在 `VIEWS` 注册表中引用。

## 页面体系（`src/pages`）
- `mod.rs`：按业务域划分子模块（convert/develop/network/snippet/toolkit 等），每个子模块下的文件实现具体 Page。
- 各 Page 遵循 `pub struct XxxPage` + `impl XxxPage { pub fn build(...) }` + `impl Render for XxxPage` 模式。`build` 负责创建状态实体（如 `InputState`），`render` 构建 UI。
- `src/pages/develop/`：开发者工具集，覆盖证书解析、哈希、加解密、二维码、随机数据等；`json.rs` 提供精简 JSON 工具（原文编辑器使用纯文本输入，格式化结果通过只读代码视图展示，仅保留“格式化 / 查看原文”按钮方便对比），演示页 `demo.rs` 保留 gpui code_editor 最小示例用于性能验证。
- `src/pages/utils.rs`：通用工具函数（字符串清洗、字节格式化）。

## 插件层（`src/plugins`）
- 对应页面的业务计算/系统调用封装，例如：
  - `hash`：文本/文件哈希计算。
  - `qrcode`：二维码生成。
  - `dns`、`system`：网络与系统信息查询。
- 页面通过调用插件函数把 UI 与底层逻辑解耦。

## 运行流程
1. `main()` 创建 `Application`，注册资源、主题、窗口。
2. `MainView::new` 构造全量页面并登记到 `VIEWS` 映射，侧边栏根据 `group` 聚类展示。
3. 用户选择菜单项时更新 `selected`，内容区通过缓存的 `AnyView` 切换页面。

## 当前待办
- 暂无明确待办；如有新需求可在此处补充。
