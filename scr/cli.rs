//! CLI-Definitionen mit clap.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// code2md - Exportiert Projektcode in eine strukturierte Markdown-Datei.
#[derive(Parser, Debug)]
#[command(
    name = "code2md",
    version,
    author,
    about = "Exportiert Projektcode in eine strukturierte Markdown-Datei",
    long_about = "code2md sammelt alle relevanten Quelldateien eines Projekts und\n\
                  exportiert sie in eine übersichtliche Markdown-Datei mit\n\
                  Ordnerstruktur und Syntax-Highlighting.",
    after_help = "BEISPIELE:\n    \
        code2md ./mein-projekt --type python\n    \
        code2md ./fullstack-app --type vue,python --output projekt.md\n    \
        code2md ./app --type react --ext .env .graphql\n    \
        code2md ./code --type node --exclude tests/ fixtures/\n    \
        code2md list-types"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Projektverzeichnis (Standard: aktuelles Verzeichnis)
    #[arg(default_value = ".")]
    pub directory: PathBuf,

    /// Projekttyp(en), kommasepariert (z.B. python,vue,config)
    #[arg(short = 't', long = "type", value_delimiter = ',')]
    pub types: Option<Vec<String>>,

    /// Zusätzliche Dateiendungen (z.B. .env .graphql)
    #[arg(short = 'e', long = "ext", num_args = 1..)]
    pub extensions: Option<Vec<String>>,

    /// Zusätzliche Ausschlüsse (Ordner/Dateien/Patterns)
    #[arg(short = 'x', long = "exclude", num_args = 1..)]
    pub excludes: Option<Vec<String>>,

    /// Ausgabedatei (Standard: <projektname>_code.md)
    #[arg(short = 'o', long = "output")]
    pub output: Option<PathBuf>,

    /// Projektname für den Header (Standard: Ordnername)
    #[arg(short = 'n', long = "name")]
    pub name: Option<String>,

    /// Ordnerstruktur-Baum nicht ausgeben
    #[arg(long = "no-tree")]
    pub no_tree: bool,

    /// Standard-Ausschlüsse deaktivieren
    #[arg(long = "no-default-excludes")]
    pub no_default_excludes: bool,

    /// Ausführliche Ausgabe
    #[arg(short = 'v', long = "verbose")]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Zeigt alle verfügbaren Projekttypen an
    ListTypes,
}

impl Cli {
    /// Prüft, ob die CLI-Argumente valide sind.
    pub fn validate(&self) -> anyhow::Result<()> {
        // Wenn ein Subcommand aktiv ist, keine weitere Validierung nötig
        if self.command.is_some() {
            return Ok(());
        }

        // Mindestens --type oder --ext muss angegeben sein
        if self.types.is_none() && self.extensions.is_none() {
            anyhow::bail!(
                "Bitte mindestens --type oder --ext angeben.\n\
                 Nutze 'code2md list-types' für verfügbare Typen."
            );
        }

        // Verzeichnis muss existieren
        if !self.directory.exists() {
            anyhow::bail!(
                "Verzeichnis '{}' existiert nicht.",
                self.directory.display()
            );
        }

        if !self.directory.is_dir() {
            anyhow::bail!(
                "'{}' ist kein Verzeichnis.",
                self.directory.display()
            );
        }

        Ok(())
    }

    /// Gibt den Projektnamen zurück (aus --name oder Ordnername).
    pub fn project_name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            self.directory
                .canonicalize()
                .ok()
                .and_then(|p| p.file_name().map(|s| s.to_string_lossy().to_string()))
                .unwrap_or_else(|| "project".to_string())
        })
    }

    /// Gibt den Ausgabepfad zurück.
    pub fn output_path(&self) -> PathBuf {
        self.output.clone().unwrap_or_else(|| {
            let safe_name: String = self
                .project_name()
                .chars()
                .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
                .collect();
            self.directory.join(format!("{}_code.md", safe_name))
        })
    }
}
