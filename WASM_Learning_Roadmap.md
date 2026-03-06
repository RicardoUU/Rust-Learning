# Rust WebAssembly（Wasm）学习路线图

本路线图适合已经开始学习 Rust，并希望进一步掌握 **Rust + WebAssembly（Wasm）开发** 的学习者。

如果你已经掌握 Rust 的基础语法、所有权、模块、错误处理等内容，那么就可以按照这份路线图逐步进入 Wasm 学习阶段。

---

## 一、什么是 WebAssembly？

WebAssembly，简称 **Wasm**，是一种可以在现代浏览器中运行的二进制格式。  
它的设计目标是让代码能够以接近原生的性能在浏览器中执行，同时还能与 JavaScript 协作。

Wasm 具有以下特点：

- 运行效率高
- 文件体积小
- 加载速度快
- 可由多种语言编译生成
- 能与 JavaScript 配合使用
- 适合高性能或计算密集型场景

Rust 是学习 Wasm 非常合适的一门语言，因为它具有：

- 高性能
- 内存安全
- 零成本抽象
- 工具链成熟
- 与 Wasm 生态结合良好

---

## 二、学习 Wasm 之前需要掌握什么？

在开始学习 Wasm 之前，建议你已经具备以下 Rust 基础：

### 1. Rust 基础语法
- 变量与常量
- 基本数据类型
- 函数
- 控制流
- 字符串和数组

### 2. 所有权与借用
- 所有权规则
- 引用和借用
- 生命周期基础概念

### 3. 模块与 Cargo
- `mod`
- `use`
- crate 结构
- `Cargo.toml`
- Cargo 基本命令

### 4. 错误处理
- `Option`
- `Result`
- `?` 运算符

### 5. 常见集合
- `Vec`
- `String`
- `HashMap`

如果你对这些内容还不熟练，建议先完成当前仓库中的 Rust 基础章节，再继续学习 Wasm。

---

## 三、为什么学习 Rust + Wasm？

学习 Rust + Wasm 的价值主要体现在以下几个方面：

### 1. 提升前端高性能计算能力
Wasm 很适合处理以下任务：

- 图像处理
- 音视频处理
- 大量数据计算
- 游戏逻辑
- 图形渲染
- 文本解析

### 2. 用 Rust 编写可在浏览器运行的逻辑
你可以把一些核心逻辑从 JavaScript 转移到 Rust 中，让代码更安全、更高效。

### 3. 拓展 Rust 的应用场景
学习 Wasm 后，Rust 不再只局限于命令行程序、后端服务或系统开发，也可以进入浏览器和前端场景。

---

## 四、推荐学习路线

建议你按照下面的顺序来学习。

---

## 第一步：理解 Wasm 的基本概念

在真正开始写代码之前，建议先理解以下问题：

- Wasm 是什么？
- Wasm 和 JavaScript 的关系是什么？
- Rust 为什么适合编译到 Wasm？
- Wasm 在浏览器中是如何运行的？
- Wasm 适合做什么，不适合做什么？

这一阶段的目标是建立整体认知，而不是马上开始写复杂代码。

---

## 第二步：安装 Wasm 开发工具链

Rust 编译到 Wasm 时，通常需要准备以下工具。

### 1. 安装 Wasm 编译目标

```bash
rustup target add wasm32-unknown-unknown
```

这个目标用于把 Rust 代码编译为 Wasm。

### 2. 安装 wasm-pack

```bash
cargo install wasm-pack
```

`wasm-pack` 是 Rust Wasm 开发中非常常用的工具，可以帮助你：

- 编译 Wasm
- 生成 JavaScript 包装代码
- 输出适用于浏览器或前端工程的构建结果

---

## 第三步：完成第一个最小 Wasm 示例

学习 Wasm 最好的起点是完成一个最小示例，建议只做以下事情：

- 在 Rust 中写一个简单函数
- 使用 `wasm-bindgen` 导出这个函数
- 在 JavaScript 中调用该函数
- 把结果显示到浏览器页面中

这个阶段的重点是理解：

- `#[wasm_bindgen]` 的作用
- `wasm-pack build --target web` 的作用
- Rust 和 JavaScript 是如何连接起来的

建议配合仓库中的示例目录一起学习：

- [Wasm 最小示例教程](./examples/wasm-hello/README.md)

---

## 第四步：掌握核心工具

学习 Rust + Wasm 时，有几个核心工具需要重点掌握。

### 1. wasm-bindgen

`wasm-bindgen` 是 Rust 和 JavaScript 互操作时的核心工具。

你需要掌握：

- 如何导出 Rust 函数给 JavaScript
- 如何接收 JavaScript 传入的参数
- 如何返回字符串、数字等基础类型
- Rust 与 JS 的基础类型映射关系

### 2. wasm-pack

`wasm-pack` 是最常见的 Rust Wasm 构建工具。

你需要掌握：

- 如何构建 Wasm 项目
- `--target web` 的作用
- `pkg/` 目录中各文件的作用
- 如何在前端页面中引用构建结果

常用命令如下：

```bash
wasm-pack build --target web
```

### 3. web-sys

`web-sys` 用于让 Rust 直接访问浏览器提供的 Web API，例如：

- `window`
- `document`
- 页面元素
- 事件监听
- DOM 操作

建议在你已经完成最小示例之后，再开始学习 `web-sys`，不要一开始就直接上复杂页面交互。

---

## 五、分阶段学习任务

为了更容易上手，建议把 Wasm 学习拆成几个阶段。

---

## 阶段一：入门阶段

### 学习目标
理解 Rust 如何编译成 Wasm，并在浏览器中运行。

### 建议练习
- 创建最小 Wasm 项目
- 编写一个 `greet()` 函数
- 在页面中显示 Rust 返回的字符串

---

## 阶段二：交互阶段

### 学习目标
掌握 JavaScript 与 Rust 之间的参数传递。

### 建议练习
- 编写 `add(a, b)` 函数
- 从 JavaScript 向 Rust 传入数字
- 将 Rust 计算结果显示到页面中

---

## 阶段三：DOM 操作阶段

### 学习目标
学习用 Rust 操作浏览器页面。

### 建议练习
- 获取页面元素
- 修改文本内容
- 绑定点击事件
- 制作一个简单计数器

---

## 阶段四：小项目阶段

### 学习目标
把 Wasm 应用到一个完整的小项目中。

### 建议项目
- 计数器
- 计算器
- Todo List
- Markdown 解析器
- Canvas 绘图示例
- 小游戏 Demo

---

## 六、常见问题

### 1. Wasm 能完全替代 JavaScript 吗？

不能。

Wasm 更适合处理：

- 高性能逻辑
- 核心计算
- 图形处理
- 数据处理
- 游戏引擎部分模块

而前端页面结构、UI 框架、生态工具等，JavaScript 仍然非常重要。

---

### 2. 为什么学习 Wasm 还需要了解一点前端？

因为浏览器中的 Wasm 通常不是单独运行的，而是与 JavaScript、HTML 配合使用。

你至少需要了解：

- HTML 基础
- JavaScript 模块
- 浏览器如何加载脚本
- 本地静态服务的基本概念

---

### 3. 为什么不能直接双击打开 HTML 文件？

因为很多浏览器在加载 Wasm 模块时要求通过 HTTP 服务访问。  
如果直接用 `file://` 打开页面，可能会加载失败。

建议使用以下方式启动本地服务：

```bash
npx serve .
```

或者：

```bash
python -m http.server 3000
```

---

## 七、推荐学习顺序总结

建议按以下顺序推进：

1. 完成 Rust 基础学习
2. 理解 Wasm 的基本概念
3. 安装 `wasm32-unknown-unknown`
4. 安装 `wasm-pack`
5. 完成最小示例
6. 学习 `wasm-bindgen`
7. 学习参数交互
8. 学习 `web-sys`
9. 完成 DOM 交互示例
10. 做一个完整小项目练手

---

## 八、结合本仓库的学习建议

建议你结合本仓库以下内容一起学习：

- Rust 基础章节
- 所有权章节
- 模块与包章节
- 错误处理章节
- [Wasm 最小示例教程](./examples/wasm-hello/README.md)

这样可以把 Rust 基础知识与 Wasm 实战逐步串联起来。

---

## 九、推荐阅读资料

建议继续阅读以下资料：

- Rust 官方文档
- Rust and WebAssembly Book
- wasm-bindgen 官方文档
- wasm-pack 官方文档

---

## 十、最终学习目标

完成本路线图后，你应该能够：

- 理解 Rust + Wasm 的基础开发流程
- 独立创建一个最小 Rust Wasm 项目
- 使用 Rust 导出函数给 JavaScript
- 完成简单页面交互
- 做出基础的小型 Wasm 项目

当你完成这些内容后，就可以继续学习更深入的方向，例如：

- Rust + Wasm + Canvas
- Rust + Wasm + 前端框架
- Rust + Wasm 性能优化
- Rust + Wasm 工程化实践