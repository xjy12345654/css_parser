<div align="center">

# 📐 CSS Unit Converter | CSS 单位转换工具

![Rust Version](1.85)
![Slint GUI](1.10.0)
![License](https://img.shields.io/badge/License-MIT-green)

**Convert `px` to `rem` or `vw/vh` with CSS parsing**  
**解析 CSS 文件，实现 px 与 rem/vw/vh 单位转换**

![Preview](./ui/images/home.png)

</div>

---

## 🌟 Features | 核心功能

- **Dual Mode Conversion**  
  **双模式转换**  
  Support `px` ->`rem` and `px`-> `vw/vh` conversions based on viewport size  
  支持根据视口尺寸进行 `px` 与 `rem` 或 `vw/vh` 单位转换

- ** CSS Parsing**  
  ** CSS 解析**  
  Powered by [lightningcss](https://github.com/parcel-bundler/lightningcss) for accurate CSS syntax analysis  
  基于高性能 lightningcss 实现精准语法解析

- **GUI**  
  Built with modern Slint framework for cross-platform compatibility  
  使用 Slint 框架构建

---

## 🚀 Quick Start | 快速开始

### Prerequisites | 环境要求

- Rust 1.85+ ([Installation Guide](https://www.rust-lang.org/tools/install))
- Cargo package manager

### Installation | 安装步骤

```bash
# Clone repository
git clone https://github.com/xjy12345654/css_parser.git
cd css-unit-converter

# Build and run
cargo run

# Build optimized release
cargo build --release

```

## 🛠 Configuration | 配置说明

### 📂 File Naming Convention | 文件命名规则

| Pattern        | Behavior              | 说明            |
| -------------- | --------------------- | --------------- |
| `*_conv_rem.*` | Skip rem conversion   | 跳过 rem 转换   |
| `*_conv_vw.*`  | Skip vw/vh conversion | 跳过 vw/vh 转换 |

---

### ✨ Special Syntax | 特殊语法

```css
/* Explicit vh unit specification | 显式指定 vh 单位 */
.element {
  width: 160px; /* Auto-convert to vw | 自动转换为 vw */
  height: vh(60); /* convert to vw 👉 转换为 vh */
  margin: 12px; /*  Auto-convert to vw  | 转换为 vw */
}
```

## ⚠️ Notes | 注意事项

### 📱 Media Query Handling | 媒体查询规则

媒体查询规则

```css
@media screen and (max-width: 300px) {
  /* Values in this scope remain unchanged | 此区域内的值保持原单位 */
  .box {
    border: 1px solid #000; /* 🚫 No conversion | 不进行单位转换 */
  }
}
/* 
Pixel values in media queries will NOT be converted
媒体查询中的像素值保留原单位
Absolute Values
绝对值处理
1px values are preserved by default
默认保留 1px 绝对值
*/
```

## 🗂 Project Structure | 项目结构

```bash
css_parser/
├── src/                # 🦀 Rust source code | Rust 源代码
│   ├── lib.rs          # 🧠 Core conversion logic | 核心转换逻辑
│   └── app.rs          # 🖥️ UI callback handlers | UI 回调处理器
├── ui/                 # 🎨 Slint GUI designs | GUI 设计文件
│   ├── app-window.slint# 🪟 Main application window | 主窗口文件
│   ├── components/     # 🧩 Reusable UI components | 可复用组件
│   ├── psd/            # 📐 Design source files | 设计源文件
│   └── images/         # 🖼️ UI assets | 界面素材
├── build.rs            # 🔨 Build script | 构建脚本
└── Cargo.toml          # 📦 Dependencies config | 依赖配置
```
