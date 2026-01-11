//! Projekttyp-Definitionen und Syntax-Highlighting-Mapping.

use std::collections::{HashMap, HashSet};

/// Definition eines Projekttyps mit zugehörigen Dateiendungen.
#[derive(Debug, Clone)]
pub struct ProjectType {
    pub name: &'static str,
    pub description: &'static str,
    pub extensions: &'static [&'static str],
    pub default_syntax: &'static str,
}

/// Alle verfügbaren Projekttypen.
pub static PROJECT_TYPES: &[ProjectType] = &[
    ProjectType {
        name: "python",
        description: "Python-Projekte",
        extensions: &[".py", ".pyi", ".pyw"],
        default_syntax: "python",
    },
    ProjectType {
        name: "arduino",
        description: "Arduino/C++ Projekte",
        extensions: &[".ino", ".cpp", ".c", ".h", ".hpp"],
        default_syntax: "cpp",
    },
    ProjectType {
        name: "vue",
        description: "Vue.js Projekte",
        extensions: &[".vue", ".js", ".ts", ".jsx", ".tsx", ".json", ".css", ".scss", ".sass", ".less"],
        default_syntax: "vue",
    },
    ProjectType {
        name: "react",
        description: "React.js Projekte",
        extensions: &[".jsx", ".tsx", ".js", ".ts", ".json", ".css", ".scss", ".sass", ".less"],
        default_syntax: "jsx",
    },
    ProjectType {
        name: "web",
        description: "Web-Projekte (HTML/CSS/JS)",
        extensions: &[".html", ".htm", ".css", ".scss", ".sass", ".less", ".js", ".ts"],
        default_syntax: "html",
    },
    ProjectType {
        name: "php",
        description: "PHP-Projekte",
        extensions: &[".php", ".phtml", ".php3", ".php4", ".php5", ".phps"],
        default_syntax: "php",
    },
    ProjectType {
        name: "node",
        description: "Node.js Projekte",
        extensions: &[".js", ".ts", ".mjs", ".cjs", ".json"],
        default_syntax: "javascript",
    },
    ProjectType {
        name: "flutter",
        description: "Flutter/Dart Projekte",
        extensions: &[".dart", ".yaml", ".json"],
        default_syntax: "dart",
    },
    ProjectType {
        name: "rust",
        description: "Rust Projekte",
        extensions: &[".rs", ".toml"],
        default_syntax: "rust",
    },
    ProjectType {
        name: "go",
        description: "Go Projekte",
        extensions: &[".go", ".mod", ".sum"],
        default_syntax: "go",
    },
    ProjectType {
        name: "java",
        description: "Java Projekte",
        extensions: &[".java", ".xml", ".gradle", ".properties"],
        default_syntax: "java",
    },
    ProjectType {
        name: "csharp",
        description: "C# Projekte",
        extensions: &[".cs", ".csproj", ".sln", ".xaml"],
        default_syntax: "csharp",
    },
    ProjectType {
        name: "config",
        description: "Konfigurationsdateien",
        extensions: &[".json", ".yaml", ".yml", ".toml", ".ini", ".cfg", ".conf", ".env"],
        default_syntax: "yaml",
    },
    ProjectType {
        name: "docs",
        description: "Dokumentationsdateien",
        extensions: &[".md", ".rst", ".txt", ".adoc"],
        default_syntax: "markdown",
    },
];

/// Standard-Ausschlüsse für Ordner und Dateien.
pub static DEFAULT_EXCLUDES: &[&str] = &[
    // Abhängigkeiten
    "node_modules",
    "vendor",
    "packages",
    ".pub-cache",
    // Python
    "__pycache__",
    ".pytest_cache",
    ".mypy_cache",
    ".ruff_cache",
    "venv",
    ".venv",
    "env",
    ".env",
    "*.egg-info",
    // Build-Ordner
    "dist",
    "build",
    "out",
    "target",
    "bin",
    "obj",
    // IDE/Editor
    ".idea",
    ".vscode",
    ".vs",
    "*.swp",
    "*.swo",
    // Versionskontrolle
    ".git",
    ".svn",
    ".hg",
    // OS
    ".DS_Store",
    "Thumbs.db",
    // Logs & temporäre Dateien
    "*.log",
    "logs",
    "tmp",
    "temp",
    ".tmp",
    // Coverage & Tests
    "coverage",
    ".coverage",
    "htmlcov",
    ".tox",
    ".nox",
];

/// Syntax-Highlighting Mapping für Dateiendungen.
pub fn get_syntax_map() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        (".py", "python"),
        (".pyi", "python"),
        (".pyw", "python"),
        (".js", "javascript"),
        (".mjs", "javascript"),
        (".cjs", "javascript"),
        (".ts", "typescript"),
        (".jsx", "jsx"),
        (".tsx", "tsx"),
        (".vue", "vue"),
        (".html", "html"),
        (".htm", "html"),
        (".css", "css"),
        (".scss", "scss"),
        (".sass", "sass"),
        (".less", "less"),
        (".json", "json"),
        (".yaml", "yaml"),
        (".yml", "yaml"),
        (".toml", "toml"),
        (".xml", "xml"),
        (".md", "markdown"),
        (".rst", "rst"),
        (".php", "php"),
        (".phtml", "php"),
        (".c", "c"),
        (".h", "c"),
        (".cpp", "cpp"),
        (".hpp", "cpp"),
        (".ino", "cpp"),
        (".rs", "rust"),
        (".go", "go"),
        (".dart", "dart"),
        (".java", "java"),
        (".kt", "kotlin"),
        (".cs", "csharp"),
        (".rb", "ruby"),
        (".sh", "bash"),
        (".bash", "bash"),
        (".zsh", "zsh"),
        (".fish", "fish"),
        (".ps1", "powershell"),
        (".sql", "sql"),
        (".graphql", "graphql"),
        (".dockerfile", "dockerfile"),
        (".ini", "ini"),
        (".cfg", "ini"),
        (".conf", "ini"),
        (".env", "dotenv"),
        (".gitignore", "gitignore"),
        (".gradle", "gradle"),
        (".properties", "properties"),
        (".sum", "text"),
        (".mod", "go"),
        (".csproj", "xml"),
        (".sln", "text"),
        (".xaml", "xml"),
        (".adoc", "asciidoc"),
        (".txt", "text"),
    ])
}

/// Findet einen Projekttyp anhand seines Namens.
pub fn find_project_type(name: &str) -> Option<&'static ProjectType> {
    PROJECT_TYPES.iter().find(|pt| pt.name == name.to_lowercase())
}

/// Sammelt alle Extensions für die angegebenen Projekttypen.
pub fn collect_extensions(type_names: &[String]) -> anyhow::Result<HashSet<String>> {
    let mut extensions = HashSet::new();
    
    for name in type_names {
        match find_project_type(name) {
            Some(pt) => {
                for ext in pt.extensions {
                    extensions.insert(ext.to_string());
                }
            }
            None => {
                anyhow::bail!(
                    "Unbekannter Projekttyp: '{}'. Nutze --list-types für verfügbare Typen.",
                    name
                );
            }
        }
    }
    
    Ok(extensions)
}

/// Ermittelt die Syntax-Highlighting-Sprache für eine Datei.
pub fn get_syntax_for_file(filename: &str) -> &'static str {
    let lower = filename.to_lowercase();
    
    // Spezialfälle für Dateien ohne Endung oder mit speziellem Namen
    if lower == "dockerfile" {
        return "dockerfile";
    }
    if lower == "makefile" {
        return "makefile";
    }
    if lower.starts_with(".env") {
        return "dotenv";
    }
    if lower == ".gitignore" {
        return "gitignore";
    }
    
    // Nach Dateiendung suchen
    let syntax_map = get_syntax_map();
    
    if let Some(dot_pos) = lower.rfind('.') {
        let ext = &lower[dot_pos..];
        if let Some(syntax) = syntax_map.get(ext) {
            return syntax;
        }
    }
    
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_project_type() {
        assert!(find_project_type("python").is_some());
        assert!(find_project_type("Python").is_some());
        assert!(find_project_type("PYTHON").is_some());
        assert!(find_project_type("unknown").is_none());
    }

    #[test]
    fn test_get_syntax_for_file() {
        assert_eq!(get_syntax_for_file("main.py"), "python");
        assert_eq!(get_syntax_for_file("app.tsx"), "tsx");
        assert_eq!(get_syntax_for_file("Dockerfile"), "dockerfile");
        assert_eq!(get_syntax_for_file(".gitignore"), "gitignore");
    }
}
