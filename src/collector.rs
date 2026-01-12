//! Datei-Sammlung und Filterung.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};

/// Ergebnis der Dateisammlung.
#[derive(Debug)]
pub struct CollectedFiles {
    pub files: Vec<PathBuf>,
    pub base_path: PathBuf,
}

/// Konfiguration für die Dateisammlung.
#[derive(Debug)]
pub struct CollectorConfig {
    pub extensions: HashSet<String>,
    pub excludes: GlobSet,
}

impl CollectorConfig {
    /// Erstellt eine neue Collector-Konfiguration.
    pub fn new(extensions: HashSet<String>, exclude_patterns: &[String]) -> anyhow::Result<Self> {
        let mut builder = GlobSetBuilder::new();
        
        for pattern in exclude_patterns {
            // Pattern normalisieren
            let normalized = if pattern.contains('/') || pattern.contains('\\') {
                pattern.replace('\\', "/")
            } else {
                format!("**/{}", pattern)
            };
            
            let glob = Glob::new(&normalized)
                .or_else(|_| Glob::new(&format!("**/{}", pattern)))?;
            builder.add(glob);
        }
        
        let excludes = builder.build()?;
        
        Ok(Self { extensions, excludes })
    }

    /// Prüft, ob eine Datei eingeschlossen werden soll.
    fn should_include(&self, path: &Path, base_path: &Path) -> bool {
        // Relative Pfad für Pattern-Matching
        let rel_path = path.strip_prefix(base_path).unwrap_or(path);
        let rel_str = rel_path.to_string_lossy();
        
        // Ausschluss-Patterns prüfen
        if self.excludes.is_match(rel_str.as_ref()) {
            return false;
        }
        
        // Auch einzelne Komponenten prüfen
        for component in rel_path.components() {
            let comp_str = component.as_os_str().to_string_lossy();
            if self.excludes.is_match(comp_str.as_ref()) {
                return false;
            }
        }
        
        // Extension prüfen
        if let Some(ext) = path.extension() {
            let ext_with_dot = format!(".{}", ext.to_string_lossy().to_lowercase());
            return self.extensions.contains(&ext_with_dot);
        }
        
        false
    }

    /// Prüft, ob ein Verzeichnis betreten werden soll.
    fn should_enter_dir(&self, entry: &DirEntry, base_path: &Path) -> bool {
        let path = entry.path();
        let rel_path = path.strip_prefix(base_path).unwrap_or(path);
        let rel_str = rel_path.to_string_lossy();
        
        // Ausschluss-Patterns prüfen
        if self.excludes.is_match(rel_str.as_ref()) {
            return false;
        }
        
        // Auch den Ordnernamen selbst prüfen
        if let Some(name) = path.file_name() {
            let name_str = name.to_string_lossy();
            if self.excludes.is_match(name_str.as_ref()) {
                return false;
            }
        }
        
        true
    }
}

/// Sammelt alle relevanten Dateien aus einem Verzeichnis.
pub fn collect_files(base_path: &Path, config: &CollectorConfig) -> anyhow::Result<CollectedFiles> {
    let base_path = base_path.canonicalize()?;
    let mut files = Vec::new();

    let walker = WalkDir::new(&base_path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                config.should_enter_dir(e, &base_path)
            } else {
                true
            }
        });

    for entry in walker.filter_map(|e| e.ok()) {
        let path = entry.path();
        
        if path.is_file() && config.should_include(path, &base_path) {
            files.push(path.to_path_buf());
        }
    }

    // Sortieren für konsistente Ausgabe
    files.sort_by(|a, b| {
        let rel_a = a.strip_prefix(&base_path).unwrap_or(a);
        let rel_b = b.strip_prefix(&base_path).unwrap_or(b);
        rel_a.to_string_lossy().to_lowercase().cmp(&rel_b.to_string_lossy().to_lowercase())
    });

    Ok(CollectedFiles { files, base_path })
}

/// Liest den Inhalt einer Datei sicher aus.
pub fn read_file_content(path: &Path) -> String {
    // Versuche verschiedene Encodings
    if let Ok(content) = std::fs::read_to_string(path) {
        return content;
    }
    
    // Fallback: Als Bytes lesen und lossy konvertieren
    match std::fs::read(path) {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(e) => format!("[Fehler: Datei konnte nicht gelesen werden - {}]", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_collect_files() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let base = dir.path();
        
        // Test-Struktur erstellen
        fs::create_dir_all(base.join("src"))?;
        fs::write(base.join("src/main.py"), "print('hello')")?;
        fs::write(base.join("src/utils.py"), "def util(): pass")?;
        fs::write(base.join("README.md"), "# Readme")?;
        
        let mut extensions = HashSet::new();
        extensions.insert(".py".to_string());
        
        let config = CollectorConfig::new(extensions, &[])?;
        let result = collect_files(base, &config)?;
        
        assert_eq!(result.files.len(), 2);
        Ok(())
    }
}
