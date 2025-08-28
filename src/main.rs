use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;
use log::info;

mod model;
mod operations;
mod util;

use crate::operations::organize;
use crate::util::{expand_tilde, load_config, save_default_config, save_config};

#[derive(Parser, Debug)]
#[command(name = "directory-curator", version, about = "Organize and curate files in macOS directories")] 
struct Cli {
	/// Path to process; overrides config
	#[arg(short = 's', long = "source", value_name = "PATH")]
	source: Option<String>,
	/// Target root path; overrides config
	#[arg(short = 't', long = "target", value_name = "PATH")]
	target: Option<String>,
	/// Dry-run only prints the plan without moving files
	#[arg(long = "dry-run", default_value_t = false)]
	dry_run: bool,
	/// Show verbose logs
	#[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
	verbose: u8,
	/// Subcommands
	#[command(subcommand)]
	command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// Print current config path and values
	Config,
	/// Generate (or overwrite) a default config file
	InitConfig,
	/// Set source directory in config
	SetSource {
		/// Source directory path
		#[arg(value_name = "PATH")]
		path: String,
	},
	/// Set target root in config
	SetTarget {
		/// Target root path
		#[arg(value_name = "PATH")]
		path: String,
	},
}

fn ensure_config() -> Result<PathBuf> {
	let Some(proj) = ProjectDirs::from("", "", "fine-directory-curator") else {
		anyhow::bail!("Unable to determine config directory")
	};
	let config_dir = proj.config_dir();
	fs::create_dir_all(config_dir)
		.with_context(|| format!("Create config dir: {}", config_dir.display()))?;
	let config_path = config_dir.join("config.toml");
	if !config_path.exists() {
		save_default_config(&config_path)?;
	}
	Ok(config_path)
}

fn main() -> Result<()> {
	let cli = Cli::parse();
	// init logger
	let log_level = match cli.verbose {
		0 => "info",
		1 => "debug",
		_ => "trace",
	};
	env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

	let config_path = ensure_config()?;

	match &cli.command {
		Some(Commands::InitConfig) => {
			save_default_config(&config_path)?;
			println!("Initialized default config at {}", config_path.display());
			return Ok(());
		}
		Some(Commands::Config) => {
			let cfg = load_config(&config_path)?;
			println!("Config path: {}\n{}", config_path.display(), toml::to_string_pretty(&cfg)?);
			return Ok(());
		}
		Some(Commands::SetSource { path }) => {
			let mut cfg = load_config(&config_path)?;
			cfg.source_dir = path.clone();
			save_config(&config_path, &cfg)?;
			println!("Source directory set to: {}", path);
			return Ok(());
		}
		Some(Commands::SetTarget { path }) => {
			let mut cfg = load_config(&config_path)?;
			cfg.target_dir = path.clone();
			save_config(&config_path, &cfg)?;
			println!("Target root set to: {}", path);
			return Ok(());
		}
		None => {}
	}

	let mut cfg = load_config(&config_path)?;

	// Check if only source or target is provided (config-only mode)
	let config_only = cli.source.is_some() && cli.target.is_none() && !cli.dry_run && cli.verbose == 0;
	
	if let Some(src) = cli.source { 
		cfg.source_dir = src; 
		if config_only {
			// Only update config, don't execute
			save_config(&config_path, &cfg)?;
			println!("Source directory updated in config to: {}", cfg.source_dir);
			return Ok(());
		}
	}
	if let Some(dst) = cli.target { cfg.target_dir = dst; }

	let source_dir = expand_tilde(&cfg.source_dir);
	let target_dir = expand_tilde(&cfg.target_dir);

	if !Path::new(&source_dir).exists() {
		anyhow::bail!("Source directory does not exist: {}", source_dir.display());
	}
	fs::create_dir_all(&target_dir)
		.with_context(|| format!("Ensure target root: {}", target_dir.display()))?;

	info!("Organizing from {} -> {}", source_dir.display(), target_dir.display());
	organize(&source_dir, &target_dir, &cfg, cli.dry_run)?;
	Ok(())
}