use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};
use cosmic_config::{Config, CosmicConfigEntry};
use cosmic_theme::{Theme, ThemeBuilder, ThemeMode};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(
    name = "cosmic-theme-import",
    version,
    about = "Lightweight CLI utility to import and export COSMIC desktop themes"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Import a theme from a .ron file and apply it to COSMIC
    Import {
        /// Path to the .ron theme file
        path: PathBuf,
        /// Do not automatically switch system between light/dark mode
        #[arg(long)]
        no_switch_mode: bool,
    },
    /// Export the currently active COSMIC theme to a .ron file
    Export {
        /// Path where the .ron theme file will be saved
        path: PathBuf,
        /// Export dark theme regardless of current system mode
        #[arg(long, conflicts_with = "light")]
        dark: bool,
        /// Export light theme regardless of current system mode
        #[arg(long, conflicts_with = "dark")]
        light: bool,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Import {
            path,
            no_switch_mode,
        } => import_theme(&path, !no_switch_mode),
        Command::Export { path, dark, light } => export_theme(&path, dark, light),
    }
}

fn import_theme(path: &Path, switch_mode: bool) -> Result<()> {
    let content = read_theme_file(path)?;
    let builder = parse_theme_builder(&content)?;
    let is_dark = builder.palette.is_dark();

    if switch_mode {
        set_system_mode(is_dark)?;
    }

    let (builder_config, theme_config) = theme_configs(is_dark)?;
    builder
        .write_entry(&builder_config)
        .context("Failed to write ThemeBuilder configuration")?;
    builder
        .build()
        .write_entry(&theme_config)
        .context("Failed to write compiled Theme configuration")?;

    println!(
        "Imported theme \"{}\" from {}",
        mode_name(is_dark),
        path.display()
    );
    Ok(())
}

fn export_theme(path: &Path, dark: bool, light: bool) -> Result<()> {
    ensure_ron_extension(path)?;
    let is_dark = if dark {
        true
    } else if light {
        false
    } else {
        current_system_mode()
    };

    let (builder_config, _) = theme_configs(is_dark)?;
    let builder = ThemeBuilder::get_entry(&builder_config).unwrap_or_else(|(_, builder)| builder);

    let ron = ron::ser::to_string_pretty(&builder, ron::ser::PrettyConfig::default())
        .context("Failed to serialize ThemeBuilder to RON")?;
    fs::write(path, ron)
        .with_context(|| format!("Failed to write theme to '{}'", path.display()))?;

    println!(
        "Exported theme \"{}\" to {}",
        mode_name(is_dark),
        path.display()
    );
    Ok(())
}

fn read_theme_file(path: &Path) -> Result<String> {
    ensure_ron_extension(path)?;
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read theme file '{}'", path.display()))
}

fn ensure_ron_extension(path: &Path) -> Result<()> {
    if path.extension().and_then(|e| e.to_str()) != Some("ron") {
        bail!("Path must have a .ron extension: {}", path.display());
    }
    Ok(())
}

fn parse_theme_builder(content: &str) -> Result<ThemeBuilder> {
    ron::de::from_str(&normalize_hex_colors(content)).context("Failed to deserialize theme")
}

fn normalize_hex_colors(content: &str) -> String {
    content
        .split('"')
        .enumerate()
        .map(|(i, chunk)| match i % 2 == 0 {
            true => chunk.to_owned(),
            false => expand_hex_color(chunk).unwrap_or_else(|| chunk.to_owned()),
        })
        .collect::<Vec<_>>()
        .join("\"")
}

fn expand_hex_color(text: &str) -> Option<String> {
    let digits = text.strip_prefix('#')?;
    (digits.len() == 6 && digits.bytes().all(|b| b.is_ascii_hexdigit()))
        .then(|| format!("#{digits}ff"))
}

fn set_system_mode(is_dark: bool) -> Result<()> {
    let mode_config = ThemeMode::config().context("Failed to get ThemeMode config")?;
    let mut mode = ThemeMode::get_entry(&mode_config).unwrap_or_default();

    if mode.is_dark != is_dark {
        mode.set_is_dark(&mode_config, is_dark)
            .context("Failed to update active dark/light mode")?;
    }
    Ok(())
}

fn current_system_mode() -> bool {
    ThemeMode::config()
        .ok()
        .and_then(|config| ThemeMode::get_entry(&config).ok().map(|mode| mode.is_dark))
        .unwrap_or(true)
}

fn theme_configs(is_dark: bool) -> Result<(Config, Config)> {
    let (builder, theme) = if is_dark {
        (ThemeBuilder::dark_config(), Theme::dark_config())
    } else {
        (ThemeBuilder::light_config(), Theme::light_config())
    };
    Ok((
        builder.context("Failed to get ThemeBuilder config")?,
        theme.context("Failed to get Theme config")?,
    ))
}

fn mode_name(is_dark: bool) -> &'static str {
    if is_dark { "dark" } else { "light" }
}
