//! # fltk-help — An out-of-the-box FLTK help page UI component.
//!
//! This crate provides a simple, ready-to-use tab that allows users to open
//! an HTML help document using their system's default browser. It is specifically
//! designed to be robust, zero-configuration, and fully compatible with
//! Windows Store / MSIX sandboxes.

#![warn(missing_docs)]

use fltk::button::Button;
use fltk::enums::Align;
use fltk::frame::Frame;
use fltk::group::{Group, Tabs};
use fltk::prelude::*;
use std::path::Path;

/// The HelpPage component.
pub struct HelpPage {
    /// The FLTK Group containing the help UI (button and text).
    pub group: Group,
    _target_url: String,
}

impl HelpPage {
    /// Creates a `HelpPage` and automatically adds it as a new tab to a `Tabs` container.
    ///
    /// This method is designed to be **zero-configuration**.
    ///
    /// # Path Resolution Rules
    /// - If `file_name` is an **absolute path** (e.g., `C:/docs/help.html`), it is used directly.
    /// - If `file_name` is a **relative path or just a file name** (e.g., `help.html`),
    ///   the library will first look for it inside the system's `AppData/Local/MyApp` directory.
    ///   If not found there, it falls back to the provided relative path.
    ///
    /// # Arguments
    /// * `_tabs` - The `Tabs` widget to add the help page to.
    /// * `tab_label` - The label for the new tab (e.g., "Help").
    /// * `file_name` - The name of the HTML file or its path.
    pub fn create_tab(_tabs: &mut Tabs, tab_label: &str, file_name: &str) -> Self {
        let target_url = Self::resolve_target_url(file_name);
        
        let mut group = Group::new(0, 25, 800, 600, tab_label);
        group.set_label(tab_label);

        let group_w = 800;

        let mut frame = Frame::new(0, 100, group_w, 50, "Click the button below to view the help document.");
        frame.set_align(Align::Center);
        frame.set_label_size(16);

        let mut btn = Button::new(300, 180, 200, 50, "Open Help Document");
        btn.set_label_size(18);

        let url_clone = target_url.clone();
        btn.set_callback(move |_| {
            if let Err(e) = webbrowser::open(&url_clone) {
                eprintln!("Failed to open browser: {}", e);
            }
        });

        // 动态居中布局
        let mut f_clone = frame.clone();
        let mut b_clone = btn.clone();
        group.resize_callback(move |_g, _x, _y, w, h| {
            f_clone.resize(0, h / 3, w, 50);
            let btn_w = 200;
            b_clone.resize((w - btn_w) / 2, h / 2, btn_w, 50);
        });

        group.end();

        Self {
            group,
            _target_url: target_url,
        }
    }

    /// 核心逻辑：根据传入的路径，自动解析出最终的真实 URL
    fn resolve_target_url(file_name: &str) -> String {
        let path = Path::new(file_name);

        // 1. 如果是绝对路径且存在，直接使用
        if path.is_absolute() && path.exists() {
            return Self::path_to_url(path);
        }

        // 2. 如果传入的是纯文件名或相对路径，优先去 AppData/Local/MyApp 下查找
        let file_name_only = path.file_name().unwrap_or_default();
        let appdata_dir = dirs::data_local_dir()
            .unwrap_or_else(std::env::temp_dir)
            .join("MyApp");
        let appdata_path = appdata_dir.join(file_name_only);

        if appdata_path.exists() {
            return Self::path_to_url(&appdata_path);
        }

        // 3. 如果 AppData 里没有，检查传入的原始相对路径（比如 "examples/help.html"）
        if path.exists() {
            return Self::path_to_url(path);
        }

        // 4. 兜底：如果哪里都找不到，返回 AppData 下的预期路径，方便用户排查
        eprintln!(
            "Warning: Help file '{}' not found. Expected at '{}'", 
            file_name, 
            appdata_path.display()
        );
        Self::path_to_url(&appdata_path)
    }

    /// 将本地绝对路径转换为 `file:///` 或 `ms-appdata:///` URL
    fn path_to_url(path: &Path) -> String {
        // 判断是否运行在 MSIX 环境中
        let is_msix = std::env::current_exe()
            .map(|p| p.to_string_lossy().to_lowercase().contains("windowsapps"))
            .unwrap_or(false);

        if is_msix {
            // 如果是 MSIX，且路径在 AppData 下，使用 ms-appdata:/// 协议
            if let Some(appdata_base) = dirs::data_local_dir() {
                if let Ok(relative_path) = path.strip_prefix(&appdata_base) {
                    let rel_str = relative_path.to_string_lossy().replace("\\", "/");
                    return format!("ms-appdata:///local/{}", rel_str);
                }
            }
        }

        // 普通环境或不在 AppData 下，回退到标准 file:/// 协议
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            let mut current = std::env::current_dir().unwrap_or_default();
            current.push(path);
            current
        };

        let mut url = abs_path.to_string_lossy().replace("\\", "/");
        if !url.starts_with("file://") {
            url = format!("file:///{}", url);
        }
        url
    }
}