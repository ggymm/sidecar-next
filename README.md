## sidecar-next

**我的开发工具箱**

### 参考代码

1. gpui：/Volumes/Data/Code/temp/gpui/zed/crates/gpui
2. gpui-component：/Volumes/Data/Code/temp/gpui/gpui-component

### 功能列表

#### 首页

- [x] 系统信息展示（用户、序列号、操作系统版本、内核版本）
- [x] 网络信息显示（网卡名称、IP地址、MAC地址）
- [x] CPU 信息展示（架构、品牌、物理/逻辑核心、频率）
- [x] 内存信息展示（已使用、总内存、交换分区、使用率）
- [x] 磁盘信息展示（类型、文件系统、容量、挂载点）

#### 转换工具

- [x] **Base64 编解码** - 支持文本双向实时转换
- [x] **时间戳转换** - Unix 时间戳与人类可读时间格式转换

#### 开发工具

- [x] **证书解析** - SSL/TLS 证书解析和验证
- [x] **哈希散列** - 支持文本和文件的多种哈希算法计算（MD5、SHA1、SHA256、SHA512）
- [x] **JSON 格式化** - JSON 数据格式化与验证，支持语法高亮
- [x] **随机数据生成** - 生成 MAC 地址、UUID v4、手机号码
- [x] **二维码** - 二维码生成和识别功能
- [ ] **加解密工具** - 各种加密解密算法工具（开发中）

#### 便捷工具

- [x] **域名查询** - DNS 解析和域名信息查询
- [ ] **文件分享** - 本地文件快速分享功能（未实现）
- [ ] **端口占用查询** - 检查端口占用状态（未实现）

#### 技术手册

- [x] **自定义手册** - 支持 Markdown 格式的自定义文档库，按名称/关键词搜索
- [ ] **代码片段库** - 代码片段存储和管理（未实现）

#### 系统配置

- [x] **macOS 设置** - 程序坞（Dock）图标大小精确调整（16-128像素）
- [ ] **Windows 设置** - Windows 系统配置（未实现）
- [x] **应用设置** - 应用程序配置和个性化设置

#### 常用命令

```bash
# 格式化
cargo fmt

# 安装包大小分析
# cargo install cargo-bloat
cargo bloat --release -n 50
cargo bloat --release --crates

# 依赖更新检查
# cargo install cargo-outdated
cargo outdated -R

# 打包
cargo clean
cargo build --release

# 更新依赖
cargo clean
rm -rf Cargo.lock
cargo update

```