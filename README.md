# Iced Web PoC

基于 Nix Flake 的 Iced Web UI 原型

## 技术栈

- **Iced** - Rust 跨平台 GUI 库 (v0.14)
- **Trunk** - Rust WASM Web 应用构建工具
- **Nix Flakes** - 开发环境管理

## 环境要求

- Nix 2.34+ with flakes enabled
- 或使用 `nix develop` 进入开发环境

## 快速开始

### 1. 进入开发环境

```bash
nix develop
```

或带 experimental features:

```bash
nix --extra-experimental-features "nix-command flakes" develop
```

### 2. 构建

```bash
trunk build
```

### 3. 运行开发服务器

```bash
trunk serve --port 8080
```

访问 http://127.0.0.1:8080

## 项目结构

```
iced_web_test/
├── flake.nix          # Nix 开发环境定义
├── flake.lock         # 锁定的依赖版本
├── Cargo.toml         # Rust 依赖配置
├── src/main.rs        # 应用代码
├── index.html         # Trunk HTML 模板
└── dist/              # 构建输出
```

## 关键依赖

在 `flake.nix` 中定义:
- `cargo` / `rustc` - Rust 工具链
- `trunk` - WASM 构建工具
- `binaryen` - WASM 优化

在 `Cargo.toml` 中配置:
- `iced` with `webgl`, `fira-sans`, `tiny-skia`, `wgpu` features

## 注意事项

- Iced 0.14 使用 `iced::application` 替代旧的 `Application` trait
- Web 端使用 `webgl` feature 提供 WebGL 渲染支持
- Trunk 自动处理 WASM 编译和资源绑定