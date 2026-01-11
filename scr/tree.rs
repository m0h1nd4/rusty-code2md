//! Generierung der Ordnerstruktur als Baum.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Repräsentiert einen Knoten im Dateibaum.
#[derive(Debug)]
struct TreeNode {
    name: String,
    is_dir: bool,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(name: String, is_dir: bool) -> Self {
        Self {
            name,
            is_dir,
            children: Vec::new(),
        }
    }

    /// Fügt einen Pfad zum Baum hinzu.
    fn add_path(&mut self, components: &[&str], is_file: bool) {
        if components.is_empty() {
            return;
        }

        let name = components[0];
        let remaining = &components[1..];
        let is_dir = !remaining.is_empty() || !is_file;

        // Existierendes Kind suchen oder neues erstellen
        let child = self.children.iter_mut().find(|c| c.name == name);
        
        match child {
            Some(existing) => {
                existing.add_path(remaining, is_file);
            }
            None => {
                let mut new_child = TreeNode::new(name.to_string(), is_dir);
                new_child.add_path(remaining, is_file);
                self.children.push(new_child);
            }
        }
    }

    /// Sortiert den Baum (Ordner zuerst, dann alphabetisch).
    fn sort(&mut self) {
        self.children.sort_by(|a, b| {
            match (a.is_dir, b.is_dir) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            }
        });
        
        for child in &mut self.children {
            child.sort();
        }
    }

    /// Rendert den Baum als Zeilen.
    fn render(&self, prefix: &str, is_last: bool, lines: &mut Vec<String>, is_root: bool) {
        if !is_root {
            let connector = if is_last { "└── " } else { "├── " };
            let suffix = if self.is_dir { "/" } else { "" };
            lines.push(format!("{}{}{}{}", prefix, connector, self.name, suffix));
        }

        let child_count = self.children.len();
        for (i, child) in self.children.iter().enumerate() {
            let is_last_child = i == child_count - 1;
            let new_prefix = if is_root {
                String::new()
            } else {
                format!("{}{}", prefix, if is_last { "    " } else { "│   " })
            };
            child.render(&new_prefix, is_last_child, lines, false);
        }
    }
}

/// Generiert eine Baumdarstellung der Ordnerstruktur.
pub fn generate_tree(files: &[PathBuf], base_path: &Path, project_name: &str) -> Vec<String> {
    // Root-Knoten erstellen
    let mut root = TreeNode::new(project_name.to_string(), true);

    // Alle Dateipfade sammeln und auch Zwischenordner hinzufügen
    let mut all_paths: HashSet<PathBuf> = HashSet::new();
    
    for file in files {
        if let Ok(rel_path) = file.strip_prefix(base_path) {
            // Datei selbst hinzufügen
            all_paths.insert(rel_path.to_path_buf());
            
            // Alle Elternordner hinzufügen
            let mut current = rel_path.to_path_buf();
            while let Some(parent) = current.parent() {
                if parent.as_os_str().is_empty() {
                    break;
                }
                all_paths.insert(parent.to_path_buf());
                current = parent.to_path_buf();
            }
        }
    }

    // Dateien zum Baum hinzufügen
    for file in files {
        if let Ok(rel_path) = file.strip_prefix(base_path) {
            let components: Vec<&str> = rel_path
                .components()
                .filter_map(|c| c.as_os_str().to_str())
                .collect();
            
            if !components.is_empty() {
                root.add_path(&components, true);
            }
        }
    }

    // Baum sortieren
    root.sort();

    // Baum rendern
    let mut lines = Vec::new();
    lines.push(format!("{}/", project_name));
    root.render("", true, &mut lines, true);

    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tree() {
        let base = PathBuf::from("/project");
        let files = vec![
            PathBuf::from("/project/src/main.py"),
            PathBuf::from("/project/src/utils/helpers.py"),
            PathBuf::from("/project/config.json"),
        ];

        let tree = generate_tree(&files, &base, "project");
        
        assert!(!tree.is_empty());
        assert!(tree[0].contains("project"));
    }
}
