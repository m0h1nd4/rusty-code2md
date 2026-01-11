//! Markdown-Dokument-Generierung.

use std::path::{Path, PathBuf};

use chrono::Local;

use crate::collector::read_file_content;
use crate::tree::generate_tree;
use crate::types::get_syntax_for_file;

/// Konfiguration für die Markdown-Generierung.
#[derive(Debug)]
pub struct MarkdownConfig {
    pub project_name: String,
    pub base_path: PathBuf,
    pub include_tree: bool,
}

/// Generiert das vollständige Markdown-Dokument.
pub fn generate_markdown(files: &[PathBuf], config: &MarkdownConfig) -> String {
    let mut lines: Vec<String> = Vec::new();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // Header
    lines.push(format!("# {}", config.project_name));
    lines.push(String::new());
    lines.push(format!("> Generiert am {}", timestamp));
    lines.push(format!("> Basisverzeichnis: `{}`", config.base_path.display()));
    lines.push(format!("> Anzahl Dateien: {}", files.len()));
    lines.push(String::new());

    // Inhaltsverzeichnis
    lines.push("## Inhaltsverzeichnis".to_string());
    lines.push(String::new());
    
    if config.include_tree {
        lines.push("1. [Ordnerstruktur](#ordnerstruktur)".to_string());
        lines.push("2. [Dateien](#dateien)".to_string());
    } else {
        lines.push("1. [Dateien](#dateien)".to_string());
    }

    for file in files {
        if let Ok(rel_path) = file.strip_prefix(&config.base_path) {
            let rel_str = rel_path.to_string_lossy();
            let anchor = generate_anchor(&rel_str);
            lines.push(format!("   - [`{}`](#{})", rel_str, anchor));
        }
    }
    lines.push(String::new());

    // Ordnerstruktur
    if config.include_tree {
        lines.push("---".to_string());
        lines.push(String::new());
        lines.push("## Ordnerstruktur".to_string());
        lines.push(String::new());
        lines.push("```".to_string());
        
        let tree = generate_tree(files, &config.base_path, &config.project_name);
        for tree_line in tree {
            lines.push(tree_line);
        }
        
        lines.push("```".to_string());
        lines.push(String::new());
    }

    // Dateien
    lines.push("---".to_string());
    lines.push(String::new());
    lines.push("## Dateien".to_string());
    lines.push(String::new());

    for file in files {
        if let Ok(rel_path) = file.strip_prefix(&config.base_path) {
            let rel_str = rel_path.to_string_lossy();
            let filename = file.file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_default();
            
            let syntax = get_syntax_for_file(&filename);
            let content = read_file_content(file);

            lines.push(format!("### `{}`", rel_str));
            lines.push(String::new());
            lines.push(format!("```{}", syntax));
            lines.push(content.trim_end().to_string());
            lines.push("```".to_string());
            lines.push(String::new());
        }
    }

    lines.join("\n")
}

/// Generiert einen Markdown-Anker aus einem Pfad.
fn generate_anchor(path: &str) -> String {
    path.chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else if c == '-' {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}

/// Formatiert eine Dateigröße menschenlesbar.
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} Bytes", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_anchor() {
        assert_eq!(generate_anchor("src/main.py"), "srcmainpy");
        assert_eq!(generate_anchor("config/settings.json"), "configsettingsjson");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 Bytes");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(2_097_152), "2.00 MB");
    }
}
