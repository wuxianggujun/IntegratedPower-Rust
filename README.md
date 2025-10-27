# IntegratedPower

一个基于 Rust 开发的现代化桌面应用程序，使用 Qt 框架构建用户界面，并利用 Polars 库进行高性能的 Excel 数据处理。

## 功能特性

- 🎨 现代化的图形用户界面
- 📊 高性能 Excel 数据处理
- 🔌 插件式处理器架构
- ⚡ 并行批量文件处理
- 🌓 浅色/深色主题支持
- 📜 操作历史记录
- ⚙️ 灵活的配置管理

## 技术栈

- **语言**: Rust (edition 2021)
- **UI 框架**: CXX-Qt (Qt 6 的 Rust 绑定)
- **数据处理**: Polars + Calamine
- **异步运行时**: Tokio
- **配置存储**: Serde + TOML

## 项目结构

```
IntegratedPower/
├── src/
│   ├── main.rs              # 应用程序入口
│   ├── app.rs               # 主应用程序逻辑
│   ├── error.rs             # 错误类型定义
│   ├── ui/                  # UI 组件
│   ├── controller/          # 控制器层
│   ├── processor/           # 处理器管理
│   ├── engine/              # 数据处理引擎
│   ├── config/              # 配置管理
│   ├── history/             # 历史记录管理
│   └── models/              # 数据模型
├── resources/               # 资源文件
└── tests/                   # 测试
```

## 构建和运行

### 前置要求

- Rust 1.70 或更高版本
- Qt 6
- CMake (用于 CXX-Qt)

### 构建

```bash
cargo build --release
```

### 运行

```bash
cargo run
```

## 开发状态

项目正在积极开发中。更多文档和功能将陆续添加。

## 许可证

待定
