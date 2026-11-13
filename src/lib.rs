//! # fltk-help — An out-of-the-box FLTK help page UI component.
//!
//! This crate provides a simple, ready-to-use tab that allows users to open
//! an HTML help document using their system's default browser. It is specifically
//! designed to be robust, zero-configuration, and fully compatible with
//! Windows Store / MSIX sandboxes.

use fltk::button::Button;
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
    /// 根据当前操作系统语言返回 UI 字符串。
    ///
    /// 返回 `(tab_label, button_label)`：
    /// - 中文系统：`("帮助", "打开帮助文档")`
    /// - 其它系统：`("Help", "Open Help Document")`
    fn get_local() -> (String, String) {
        // 优先读取 LC_ALL / LC_MESSAGES / LANG 环境变量
        let lang = std::env::var("LC_ALL")
            .or_else(|_| std::env::var("LC_MESSAGES"))
            .or_else(|_| std::env::var("LANG"))
            .unwrap_or_default()
            .to_lowercase();

        // 环境变量未提供时，再按平台查询系统语言
        let is_chinese = if !lang.is_empty() && lang != "c" && lang != "posix" {
            lang.starts_with("zh")
        } else {
            Self::detect_system_language_is_chinese()
        };

        if is_chinese {
            ("帮助".to_string(), "打开帮助文档".to_string())
        } else {
            ("Help".to_string(), "Open Help Document".to_string())
        }
    }

    /// 按平台检测系统语言是否为中文。
    #[cfg(target_os = "windows")]
    fn detect_system_language_is_chinese() -> bool {
        // 通过 PowerShell 读取系统 UI 语言
        use std::process::Command;
        if let Ok(output) = Command::new("powershell")
            .args(["-NoProfile", "-Command", "(Get-UICulture).Name"])
            .output()
        {
            let name = String::from_utf8_lossy(&output.stdout).to_lowercase();
            return name.trim().starts_with("zh");
        }
        false
    }

    #[cfg(target_os = "macos")]
    fn detect_system_language_is_chinese() -> bool {
        // defaults read -g AppleLanguages 输出形如 ( "zh-Hans", "en" )
        use std::process::Command;
        if let Ok(output) = Command::new("defaults")
            .args(["read", "-g", "AppleLanguages"])
            .output()
        {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            // 取列表中第一项
            return text.contains("zh-hans") || text.contains("zh-hant") || text.contains("\"zh");
        }
        false
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    fn detect_system_language_is_chinese() -> bool {
        // 依次尝试 locale 命令与 /etc/default/locale
        use std::process::Command;
        if let Ok(output) = Command::new("locale").output() {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            for line in text.lines() {
                if let Some(value) = line.strip_prefix("lang=") {
                    if value.contains("zh") {
                        return true;
                    }
                }
            }
        }
        if let Ok(text) = std::fs::read_to_string("/etc/default/locale") {
            return text.to_lowercase().contains("zh");
        }
        false
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", unix)))]
    fn detect_system_language_is_chinese() -> bool {
        false
    }

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

        // 根据系统语言获取本地化字符串
        let (local_tab_label, local_btn_label) = Self::get_local();

        // 若调用方传入的 tab_label 为空，则使用本地化标签
        let final_tab_label: &str = if tab_label.trim().is_empty() {
            local_tab_label.as_str()
        } else {
            tab_label
        };

        let mut group = Group::new(0, 25, 800, 600, Some(final_tab_label));
        group.set_label(final_tab_label);

        let mut btn = Button::new(300, 180, 200, 50, Some(local_btn_label.as_str()));
        btn.set_label_size(18);

        let url_clone = target_url.clone();
        btn.set_callback(move |_| {
            if let Err(e) = webbrowser::open(&url_clone) {
                eprintln!("Failed to open browser: {}", e);
            }
        });

        // 动态居中布局
        let mut b_clone = btn.clone();
        group.resize_callback(move |_g, _x, _y, w, h| {
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