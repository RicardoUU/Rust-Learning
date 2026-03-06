# Wasm 最小示例

这个示例用于帮助你完成第一个 Rust + WebAssembly 项目。

## 学习目标

- 了解 Rust 编译到 Wasm 的基本流程
- 学会安装 `wasm-pack`
- 学会使用 `wasm-bindgen` 导出函数
- 运行一个最小可用示例

## 目录结构

- `hello-wasm/`：最小 Rust Wasm 项目

## 前置准备

确保你已经安装：

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## 运行步骤

### 1. 进入项目目录

```bash
cd examples/wasm-hello/hello-wasm
```

### 2. 构建 Wasm 包

```bash
wasm-pack build --target web
```

构建完成后会生成 `pkg/` 目录。

### 3. 你将学到什么

在这个最小示例中，你会看到：
- Rust 函数如何导出到 JavaScript
- `wasm-bindgen` 的最基本使用方式
- 如何为后续 DOM 交互示例打基础

## 下一步建议

完成本示例后，你可以继续扩展：
- 暴露多个函数
- 与 JavaScript 进行参数交互
- 引入 `web-sys` 操作 DOM
- 制作一个按钮点击计数器页面
