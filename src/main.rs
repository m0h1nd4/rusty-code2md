//! code2md - Exportiert Projektcode in eine strukturierte Markdown-Datei.
//!
//! Dieses Tool sammelt alle relevanten Quelldateien eines Projekts und
//! exportiert sie in eine übersichtliche Markdown-Datei mit Ordnerstruktur
//! und Syntax-Highlighting.

mod cli;
mod collector;
mod markdown;
mod tree;
mod types;

use std::collections::HashSet;
use std::fs;

use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use cli::{Cli, Commands};
use collector::{collect_files, CollectorConfig};
use markdown::{format_size, generate_markdown, MarkdownConfig};
use types::{collect_extensions, DEFAULT_EXCLUDES, PROJECT_TYPES};

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Fehler:".red().bold(), e);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    // Subcommand verarbeiten
    if let Some(Commands::ListTypes) = cli.command {
        print_types();
        return Ok(());
    }

    // CLI validieren
    cli.validate()?;

    // Extensions sammeln
    let mut extensions: HashSet<String> = HashSet::new();

    if let Some(ref type_names) = cli.types {
        extensions.extend(collect_extensions(type_names)?);
    }

    // Zusätzliche Extensions hinzufügen
    if let Some(ref exts) = cli.extensions {
        for ext in exts {
            let normalized = if ext.starts_with('.') {
                ext.to_lowercase()
            } else {
                format!(".{}", ext.to_lowercase())
            };
            extensions.insert(normalized);
        }
    }

    // Excludes zusammenstellen
    let mut exclude_patterns: Vec<String> = if cli.no_default_excludes {
        Vec::new()
    } else {
        DEFAULT_EXCLUDES.iter().map(|s| s.to_string()).collect()
    };

    if let Some(ref excludes) = cli.excludes {
        exclude_patterns.extend(excludes.iter().cloned());
    }

    // Konfiguration
    let project_name = cli.project_name();
    let output_path = cli.output_path();
    let base_path = cli.directory.canonicalize()?;

    // Verbose Header
    if cli.verbose {
        println!();
        println!("{}", "═".repeat(60).bright_blue());
        println!("{}", "code2md - Projekt-Export".bright_blue().bold());
        println!("{}", "═".repeat(60).bright_blue());
        println!("Projektverzeichnis: {}", base_path.display());
        println!("Projektname:        {}", project_name);
        println!("Ausgabedatei:       {}", output_path.display());
        println!(
            "Dateiendungen:      {}",
            extensions.iter().cloned().collect::<Vec<_>>().join(", ")
        );
        println!("Ausschlüsse:        {} Patterns", exclude_patterns.len());
        println!("{}", "═".repeat(60).bright_blue());
        println!();
    }

    // Dateien sammeln
    println!("{}", "Sammle Dateien...".dimmed());
    
    let config = CollectorConfig::new(extensions, &exclude_patterns)?;
    let collected = collect_files(&base_path, &config)?;

    if collected.files.is_empty() {
        anyhow::bail!("Keine passenden Dateien gefunden.");
    }

    println!(
        "Gefunden: {} {}",
        collected.files.len().to_string().green().bold(),
        if collected.files.len() == 1 { "Datei" } else { "Dateien" }
    );

    // Verbose: Dateien auflisten
    if cli.verbose {
        for file in &collected.files {
            if let Ok(rel) = file.strip_prefix(&base_path) {
                println!("  {} {}", "─".dimmed(), rel.display());
            }
        }
    }

    // Markdown generieren
    println!("{}", "Generiere Markdown...".dimmed());

    let md_config = MarkdownConfig {
        project_name: project_name.clone(),
        base_path: base_path.clone(),
        include_tree: !cli.no_tree,
    };

    let markdown = generate_markdown(&collected.files, &md_config);

    // Ausgabe schreiben
    fs::write(&output_path, &markdown)?;

    // Statistik
    let file_size = fs::metadata(&output_path)?.len();
    let size_str = format_size(file_size);

    println!();
    println!("{}", "✓ Export abgeschlossen!".green().bold());
    println!("  Datei:   {}", output_path.display());
    println!("  Größe:   {}", size_str);
    println!("  Dateien: {}", collected.files.len());

    Ok(())
}

/// Gibt alle verfügbaren Projekttypen aus.
fn print_types() {
    println!();
    println!("{}", "Verfügbare Projekttypen:".bright_blue().bold());
    println!();

    for pt in PROJECT_TYPES {
        let exts = pt.extensions.join(", ");
        println!(
            "  {:<12} {}",
            pt.name.green().bold(),
            pt.description
        );
        println!(
            "  {:<12} Endungen: {}",
            "",
            exts.dimmed()
        );
        println!();
    }
}
