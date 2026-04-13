# Iced Web PoC

基于 Nix Flake 的 Iced Web UI 原型演示项目

## 项目简介

这是一个展示如何使用 Rust 的 [Iced](https://iced.rs/) GUI 库构建 WebAssembly 应用的原型项目。通过 Nix Flakes 提供可重现的开发环境，使用 [Trunk](https://trunkrs.dev/) 作为构建工具。

## 功能特性

- **跨平台 GUI**: 使用 Iced 0.14 构建的计数器应用
- **WebAssembly**: 编译为 WASM 在浏览器中运行
- **WebGL 渲染**: 使用 wgpu 后端和 WebGL 支持
- **Nix 开发环境**: 通过 Flakes 实现可重现的开发环境

## 技术栈

| 组件 | 版本 | 说明 |
|------|------|------|
| Iced | 0.14 | Rust 跨平台 GUI 库 |
| Trunk | latest | Rust WASM Web 应用构建工具 |
| Nix | 2.34+ | 包管理器与开发环境 |
| wgpu | - | GPU 渲染后端 |

### Cargo Features

- `webgl` - WebGL 渲染支持
- `fira-sans` - 字体支持
- `tiny-skia` - 2D 图形渲染
- `wgpu` - GPU 渲染后端

## 环境要求

- **Nix**: 2.34+ with flakes enabled
- **操作系统**: Linux (已在 flake.nix 中配置多系统支持)

## 快速开始

### 方式一：使用 Nix Flakes（推荐）

```bash
# 进入开发环境
nix develop

# 或指定 experimental features
nix --extra-experimental-features "nix-command flakes" develop

# 交互式模式（保存 shell 环境）
nix develop -i
```

### 方式二：手动配置环境

需要安装以下工具：
- Rust (stable toolchain)
- Trunk: `cargo install trunk`
- binaryen
- wasm-bindgen-cli
- lld (LLVM linker)

## 构建与运行

### 构建 WASM 应用

```bash
# 使用 trunk 构建
trunk build
```

构建产物将输出到 `dist/` 目录：
- `index.html` - 入口 HTML
- `iced-web-poc-*.js` - JavaScript 胶水代码
- `iced-web-poc-*.wasm` - WebAssembly 二进制

### 启动开发服务器

```bash
# 启动服务器（默认端口 8080）
trunk serve

# 指定端口
trunk serve --port 8080
```

访问 http://127.0.0.1:8080 查看应用。

### 一键构建并运行

```bash
nix develop -i -c trunk build && nix develop -i -c trunk serve --port 8080
```

### 生产构建优化

项目已配置 Release 优化：
- `opt-level = "s"` - 最小化二进制大小
- `LTO = true` - 链接时优化
- WASM 级别优化通过 Trunk 的 `data-wasm-opt="z"` 参数启用

## 项目结构

```
iced-web-poc/
├── flake.nix          # Nix 开发环境定义
├── flake.lock         # 锁定的依赖版本
├── Cargo.toml         # Rust 依赖配置
├── src/
│   └── main.rs        # 应用主代码
├── index.html         # Trunk HTML 模板
└── dist/              # 构建输出（构建后生成）
```

## 代码说明

### main.rs

应用包含以下组件：
- **Message 枚举**: 定义 `Increment` 和 `Decrement` 两种消息
- **update 函数**: 处理消息，更新状态
- **view 函数**: 渲染 UI 组件（计数器显示 + 加减按钮）
- **main 函数**: 使用 `iced::application` 启动应用

### 关键实现细节

- 使用 Iced 0.14 的新 `application` API（替代旧的 `Application` trait）
- Web 端自动启用 WebGL 渲染
- Trunk 自动处理 WASM 编译和资源绑定

## 常见问题

### Q: 构建失败，提示缺少 `libxkbcommon`？

A: 在 flake.nix 中已包含此依赖。请确保使用 `nix develop` 进入开发环境。

### Q: 浏览器中无法渲染？

A: 确保使用支持 WebGL 的浏览器。检查浏览器控制台是否有错误信息。

### Q: 如何自定义应用名称？

A: 修改 `Cargo.toml` 中的 `name` 字段，同时更新 `index.html` 中的 `<title>` 标签。

## 相关资源

- [Iced 官方文档](https://docs.iced.rs/)
- [Trunk 文档](https://trunkrs.dev/)
- [Rust WASM 手册](https://rustwasm.github.io/book/)
- [Nix Flakes 文档](https://nixos.org/manual/nix/stable/command-ref/new-cli/nix3-flake.html)

## 许可证

MIT License