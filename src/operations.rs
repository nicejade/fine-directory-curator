use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::collections::HashSet;

use anyhow::{Context, Result};
use chrono::{DateTime, Local, Datelike};
use log::info;

use crate::model::{Category, Config, Level};
use crate::util::ensure_dir;

pub fn organize(source_dir: &Path, target_dir: &Path, cfg: &Config, dry_run: bool) -> Result<()> {
	let entries = fs::read_dir(source_dir)
		.with_context(|| format!("Read dir: {}", source_dir.display()))?;

	// 收集所有需要的目录路径，避免重复创建
	let mut needed_dirs = HashSet::new();
	let mut file_operations = Vec::new();

	// 第一遍：收集所有需要的目录和文件操作
	for entry in entries {
		let entry = entry?;
		let path = entry.path();
		let file_name = entry.file_name();
		let file_name_str = file_name.to_string_lossy();

		if file_name_str.starts_with('.') { continue; }

		let is_dir = entry.file_type()?.is_dir();
		let category = if is_dir { Category::Directory } else { detect_category(&path, cfg) };
		let year = get_year(&path).unwrap_or_else(|| Local::now().year());

		let mut dest = PathBuf::from(target_dir);
		for level in &cfg.sort_rule.order {
			match level {
				Level::Year => dest.push(format!("{}", year)),
				Level::Category => dest.push(category.dir_name()),
			}
		}

		// 记录需要的目录
		needed_dirs.insert(dest.clone());
		
		// 记录文件操作
		file_operations.push((path, dest, file_name_str.to_string()));
	}

	// 第二遍：按需创建目录并执行文件操作
	for (source_path, dest_dir, file_name_str) in file_operations {
		// 只在需要时创建目录
		if !dest_dir.exists() {
			ensure_dir(&dest_dir)?;
		}

		let target_path = unique_target(&dest_dir, &file_name_str)?;

		if dry_run {
			println!("[DRY-RUN] {} -> {}", source_path.display(), target_path.display());
			continue;
		}

		info!("Move {} -> {}", source_path.display(), target_path.display());
		match fs::rename(&source_path, &target_path) {
			Ok(_) => {},
			Err(e) if e.kind() == io::ErrorKind::Other => {
				// Cross-device rename failed, fallback to copy + delete
				copy_recursively(&source_path, &target_path)?;
				remove_file_or_dir(&source_path)?;
			}
			Err(e) => {
				let err = e;
				return Err(anyhow::Error::from(err))
					.context(format!("Move {} -> {}", source_path.display(), target_path.display()));
			}
		}
	}

	// 如果不是dry-run模式，清理可能创建的空目录
	if !dry_run {
		cleanup_empty_dirs(target_dir, &cfg.sort_rule.order)?;
	}

	Ok(())
}

/// 清理空的目录，只保留有内容的目录
fn cleanup_empty_dirs(root: &Path, _sort_order: &[crate::model::Level]) -> Result<()> {
	// 从最深层开始向上清理，避免删除非空目录
	let mut dirs_to_check = Vec::new();
	
	// 收集所有可能的目录路径
	if let Ok(entries) = fs::read_dir(root) {
		for entry in entries {
			if let Ok(entry) = entry {
				let path = entry.path();
				if path.is_dir() {
					dirs_to_check.push(path);
				}
			}
		}
	}

	// 按深度排序，最深的先处理
	dirs_to_check.sort_by(|a, b| {
		let depth_a = a.components().count();
		let depth_b = b.components().count();
		depth_b.cmp(&depth_a) // 降序，最深的先处理
	});

	// 清理空目录
	for dir in dirs_to_check {
		if is_dir_empty(&dir)? {
			if let Err(e) = fs::remove_dir(&dir) {
				log::warn!("Failed to remove empty directory {}: {}", dir.display(), e);
			} else {
				log::debug!("Removed empty directory: {}", dir.display());
			}
		}
	}

	Ok(())
}

/// 检查目录是否为空（不包含任何文件或非空子目录）
fn is_dir_empty(dir: &Path) -> Result<bool> {
	let mut has_content = false;
	
	if let Ok(entries) = fs::read_dir(dir) {
		for entry in entries {
			if let Ok(entry) = entry {
				let path = entry.path();
				if path.is_file() {
					has_content = true;
					break;
				} else if path.is_dir() {
					// 递归检查子目录
					if !is_dir_empty(&path)? {
						has_content = true;
						break;
					}
				}
			}
		}
	}
	
	Ok(!has_content)
}

fn detect_category(path: &Path, cfg: &Config) -> Category {
	match path.extension().and_then(|s| s.to_str()) {
		Some(ext) => Category::from_extension(ext, &cfg.extension_overrides),
		None => Category::Others,
	}
}

fn get_year(path: &Path) -> Option<i32> {
	let meta = fs::metadata(path).ok()?;
	// On macOS, created time may exist
	if let Ok(time) = meta.created() {
		let dt: DateTime<Local> = time.into();
		return Some(dt.year());
	}
	if let Ok(time) = meta.modified() {
		let dt: DateTime<Local> = time.into();
		return Some(dt.year());
	}
	None
}

fn unique_target(dest_dir: &Path, original_name: &str) -> Result<PathBuf> {
	let mut candidate = dest_dir.join(original_name);
	if !candidate.exists() {
		return Ok(candidate);
	}
	let (stem, ext) = split_name(original_name);
	let mut index: u32 = 1;
	loop {
		let file_name = if let Some(ext) = ext.as_deref() {
			format!("{} ({}).{}", stem, index, ext)
		} else {
			format!("{} ({})", stem, index)
		};
		candidate = dest_dir.join(file_name);
		if !candidate.exists() { return Ok(candidate); }
		index += 1;
	}
}

fn split_name(name: &str) -> (String, Option<String>) {
	match name.rsplit_once('.') {
		Some((s, e)) => (s.to_string(), Some(e.to_string())),
		None => (name.to_string(), None),
	}
}

fn copy_recursively(from: &Path, to: &Path) -> Result<()> {
	if from.is_dir() {
		fs::create_dir_all(to)?;
		for entry in fs::read_dir(from)? {
			let entry = entry?;
			let src = entry.path();
			let dst = to.join(entry.file_name());
			if entry.file_type()?.is_dir() {
				copy_recursively(&src, &dst)?;
			} else {
				fs::copy(&src, &dst).with_context(|| format!("Copy {} -> {}", src.display(), dst.display()))?;
			}
		}
		Ok(())
	} else {
		if let Some(parent) = to.parent() { fs::create_dir_all(parent)?; }
		fs::copy(from, to).with_context(|| format!("Copy {} -> {}", from.display(), to.display()))?;
		Ok(())
	}
}

fn remove_file_or_dir(path: &Path) -> io::Result<()> {
	if path.is_dir() { fs::remove_dir_all(path) } else { fs::remove_file(path) }
} 