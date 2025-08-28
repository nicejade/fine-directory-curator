use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::model::Config;

pub fn expand_tilde(path: &str) -> PathBuf {
	if let Some(stripped) = path.strip_prefix("~/") {
		if let Some(home) = dirs::home_dir() {
			return home.join(stripped);
		}
	}
	PathBuf::from(path)
}

pub fn load_config(path: &Path) -> Result<Config> {
	let content = fs::read_to_string(path)
		.with_context(|| format!("Read config: {}", path.display()))?;
	let cfg: Config = toml::from_str(&content)
		.with_context(|| format!("Parse TOML config: {}", path.display()))?;
	Ok(cfg)
}

pub fn save_default_config(path: &Path) -> Result<()> {
	let default_cfg = Config::default();
	let toml_str = toml::to_string_pretty(&default_cfg)?;
	if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
	let mut f = fs::File::create(path)
		.with_context(|| format!("Create config file: {}", path.display()))?;
	f.write_all(toml_str.as_bytes())?;
	Ok(())
}

pub fn ensure_dir(path: &Path) -> Result<()> {
	fs::create_dir_all(path)
		.with_context(|| format!("Create directory: {}", path.display()))
}

pub fn save_config(path: &Path, config: &Config) -> Result<()> {
	let toml_str = toml::to_string_pretty(config)?;
	if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
	let mut f = fs::File::create(path)
		.with_context(|| format!("Create config file: {}", path.display()))?;
	f.write_all(toml_str.as_bytes())?;
	Ok(())
} 