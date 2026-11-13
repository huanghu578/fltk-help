//! Example using an absolute path (best for local development).
//!
//! Run with:
//! ```sh
//! cargo run --example absolute_path
//! ```

use fltk::{app, prelude::*, group::{Tabs, Group}, window::Window, frame::Frame};
use fltk_help::HelpPage;

fn main() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 900, 650, "My Multi-Tab App - Absolute Path Test");
    win.set_color(fltk::enums::Color::White);

    let mut tabs = Tabs::new(10, 10, 880, 630, "");

    // Tab 1: Settings
    let mut tab1 = Group::new(0, 25, 880, 605, "Settings");
    tab1.set_label("Settings");
    let mut label = Frame::new(50, 50, 300, 50, "This is Settings Page");
    label.set_label_size(24);
    tab1.end();

    // Tab 2: Help
    // 使用 CARGO_MANIFEST_DIR 获取项目根目录的绝对路径，强制使用本地文件
    let abs_path = format!("{}/examples/help.html", env!("CARGO_MANIFEST_DIR"));
    println!("Using absolute path: {}", abs_path);

    let _help = HelpPage::create_tab(&mut tabs, "Help", &abs_path);

    tabs.end();
    win.end();
    win.make_resizable(true);
    win.show();

    app.run().unwrap();
}