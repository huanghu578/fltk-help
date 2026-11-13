//! Example simulating a deployed app (best for MSIX or production).
//!
//! Run with:
//! ```sh
//! cargo run --example appdata
//! ```

use fltk::{app, prelude::*, group::{Tabs, Group}, window::Window, frame::Frame};
use fltk_help::HelpPage;

fn main() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 900, 650, "My Multi-Tab App - AppData Test");
    win.set_color(fltk::enums::Color::White);

    let mut tabs = Tabs::new(10, 10, 880, 630, "");

    // Tab 1: Settings
    let mut tab1 = Group::new(0, 25, 880, 605, "Settings");
    tab1.set_label("Settings");
    let mut label = Frame::new(50, 50, 300, 50, "This is Settings Page");
    label.set_label_size(24);
    tab1.end();

    // Tab 2: Help
    // 只传入纯文件名。库会优先去 AppData/Local/MyApp/help.html 查找。
    // 如果找不到，它会自动回退去找当前目录下的 help.html。
    let _help = HelpPage::create_tab(&mut tabs, "Help", "examples/help.html");

    tabs.end();
    win.end();
    win.make_resizable(true);
    win.show();

    app.run().unwrap();
}