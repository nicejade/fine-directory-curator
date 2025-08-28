use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
	pub source_dir: String,
	pub target_dir: String,
	/// Sorting rule: first by year, then by category type by default
	pub sort_rule: SortRule,
	/// User-defined category overrides: extension (without dot) -> Category name
	pub extension_overrides: BTreeMap<String, String>,
}

impl Default for Config {
	fn default() -> Self {
		Self {
			source_dir: "~/Downloads".to_string(),
			target_dir: "~/Documents/Matrixs".to_string(),
			sort_rule: SortRule::default(),
			extension_overrides: BTreeMap::new(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortRule {
	pub order: Vec<Level>,
}

impl Default for SortRule {
	fn default() -> Self {
		Self { order: vec![Level::Year, Level::Category] }
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Level {
	Year,
	Category,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
	Images,
	Pdfs,
	Videos,
	Audio,
	Archives,
	Documents,
	Spreadsheets,
	Presentations,
	Code,
	Design,
	MindMaps,
	Executables,
	Installers,
	Fonts,
	Others,
	Directory,
}

impl Category {
	pub fn from_extension(ext: &str, overrides: &BTreeMap<String, String>) -> Category {
		if let Some(custom) = overrides.get(&ext.to_ascii_lowercase()) {
			return match custom.as_str() {
				"images" => Category::Images,
				"pdfs" => Category::Pdfs,
				"videos" => Category::Videos,
				"audio" => Category::Audio,
				"archives" => Category::Archives,
				"documents" => Category::Documents,
				"spreadsheets" => Category::Spreadsheets,
				"presentations" => Category::Presentations,
				"code" => Category::Code,
				"design" => Category::Design,
				"mindmaps" => Category::MindMaps,
				"executables" => Category::Executables,
				"installers" => Category::Installers,
				"fonts" => Category::Fonts,
				_ => Category::Others,
			}
		}
		let e = ext.to_ascii_lowercase();
		match e.as_str() {
			// images
			"jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "webp" | "heic" | "raw" | "svg" => Category::Images,
			// pdfs
			"pdf" => Category::Pdfs,
			// videos
			"mp4" | "mov" | "m4v" | "avi" | "mkv" | "webm" | "wmv" | "flv" => Category::Videos,
			// audio
			"mp3" | "aac" | "wav" | "flac" | "m4a" | "ogg" | "aiff" => Category::Audio,
			// archives
			"zip" | "rar" | "7z" | "tar" | "gz" | "bz2" | "xz" | "dmg" => Category::Archives,
			// documents
			"doc" | "docx" | "rtf" | "txt" | "md" | "pages" => Category::Documents,
			// spreadsheets
			"xls" | "xlsx" | "numbers" | "csv" | "tsv" => Category::Spreadsheets,
			// presentations
			"ppt" | "pptx" | "key" => Category::Presentations,
			// code
			"rs" | "py" | "js" | "ts" | "tsx" | "java" | "go" | "rb" | "c" | "cpp" | "h" | "hpp" | "swift" | "kt" | "kts" | "sh" | "zsh" | "sql" | "yml" | "yaml" | "toml" | "json" | "xml" | "html" | "css" | "scss" => Category::Code,
			// design
			"psd" | "ai" | "xd" | "sketch" | "fig" => Category::Design,
			// mind maps
			"xmind" | "xmmap" => Category::MindMaps,
			// executables/installers
			"app" | "pkg" | "exe" | "msi" => Category::Executables,
			"iso" | "img" => Category::Installers,
			// fonts
			"ttf" | "otf" | "ttc" => Category::Fonts,
			_ => Category::Others,
		}
	}

	pub fn dir_name(&self) -> &'static str {
		match self {
			Category::Images => "Images",
			Category::Pdfs => "PDFs",
			Category::Videos => "Videos",
			Category::Audio => "Audio",
			Category::Archives => "Archives",
			Category::Documents => "Documents",
			Category::Spreadsheets => "Spreadsheets",
			Category::Presentations => "Presentations",
			Category::Code => "Code",
			Category::Design => "Design",
			Category::MindMaps => "MindMaps",
			Category::Executables => "Executables",
			Category::Installers => "Installers",
			Category::Fonts => "Fonts",
			Category::Others => "Others",
			Category::Directory => "Directory",
		}
	}
} 