# fltk-help

一个开箱即用的 FLTK 帮助页面 UI 组件。它提供一个带有“打开文档”按钮的 Tab 页，点击后会调用系统默认浏览器打开指定的 HTML 文档。完美兼容普通桌面程序和 MSIX 打包环境。

## 特性

- **零配置**：只需传入文件名或路径，组件会自动解析出最终的真实 URL。
- **智能降级策略**：
  1. 如果你传入的是**绝对路径**，它直接使用（非常适合本地开发和测试）。
  2. 如果你传入的是**相对路径或纯文件名**（如 `"help.html"`），它会优先去系统的 `AppData` 目录下查找。找到就直接打开（非常适合 MSIX 部署环境）；找不到则回退到本地的相对路径。
- **完美兼容 MSIX**：自动检测 MSIX 沙盒环境，并在底层自动将 `file:///` 协议切换为 `ms-appdata:///` 专属协议。
- **零 WebView 依赖**：通过调用系统浏览器打开，彻底避免了在 FLTK 中嵌入原生 WebView 所导致的 Z-Order 遮挡、高 DPI 缩放错位以及底层闪退等顽疾。

## 安装

```toml
[dependencies]
fltk = "1.5"
fltk-help = "0.1.0"
```

## 用法

只需传入你的 `Tabs` 容器、Tab 标签名，以及文件名或路径即可。

```rust
use fltk::{app, prelude::*, group::{Tabs, Group}, window::Window, frame::Frame};
use fltk_help::HelpPage;

fn main() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 900, 650, "我的应用");

    let mut tabs = Tabs::new(10, 10, 880, 630, "");

    // 你正常的页面
    let mut tab1 = Group::new(0, 25, 880, 605, "Settings");
    tab1.set_label("Settings");
    tab1.end();

    // 帮助页（只需一行代码！）
    // 直接传入相对路径、纯文件名或绝对路径，库会自动处理剩下的所有逻辑。
    let _help = HelpPage::create_tab(&mut tabs, "Help", "examples/help.html");

    tabs.end();
    win.end();
    win.show();

    app.run().unwrap();
}
```

## 许可证

以下两种许可证任选其一：
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))