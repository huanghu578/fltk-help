# fltk-help

An out-of-the-box FLTK help page UI component. It provides a ready-to-use tab with a button that opens an HTML document via the system's default browser. It is fully compatible with standard Windows applications and Windows Store / MSIX packaging.

## Features

- **Zero Configuration**: Just pass the file name or path. The component automatically resolves the final URL.
- **Smart Fallback Strategy**: 
  1. If you pass an **absolute path**, it uses it directly (perfect for local development and testing).
  2. If you pass a **relative path or just a file name** (e.g., `"help.html"`), it checks the system's `AppData` directory first. If found, it opens it (perfect for MSIX deployment). If not found, it falls back to the local relative path.
- **MSIX Compatible**: Automatically detects MSIX environments and uses the `ms-appdata:///` protocol instead of `file:///`.
- **Zero WebView Dependencies**: By using the system browser, it completely avoids the Z-order, DPI scaling, and crash issues associated with embedding native WebViews in FLTK.

## Installation

```toml
[dependencies]
fltk = "1.5"
fltk-help = "0.1.0"
```

## Usage

Simply pass your `Tabs` container, the tab label, and the file name or path.

```rust
use fltk::{app, prelude::*, group::{Tabs, Group}, window::Window, frame::Frame};
use fltk_help::HelpPage;

fn main() {
    let app = app::App::default();
    let mut win = Window::new(100, 100, 900, 650, "My App");

    let mut tabs = Tabs::new(10, 10, 880, 630, "");

    // Your normal pages
    let mut tab1 = Group::new(0, 25, 880, 605, "Settings");
    tab1.set_label("Settings");
    tab1.end();

    // The Help Page (One line does it all!)
    // Pass a relative path, file name, or absolute path. The library handles the rest.
    let _help = HelpPage::create_tab(&mut tabs, "Help", "examples/help.html");

    tabs.end();
    win.end();
    win.show();

    app.run().unwrap();
}
```

## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))