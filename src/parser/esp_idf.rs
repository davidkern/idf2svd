use std::fs;
use std::path::{Path, PathBuf};
use crate::parser;


/// Paths to all ESP IDF source files used by this module
#[derive(Default, Debug, Clone)]
pub struct EspIdfPaths {
    pub base: PathBuf,
    pub component: PathBuf,
    pub soc_component: PathBuf,
    pub soc_esp32_include: PathBuf,
}

/// Data extracted from the ESP IDF source files
#[derive(Default, Debug, Clone)]
pub struct EspIdf {
    paths: EspIdfPaths,
    soc: parser::soc::Soc,
}

/// Constructs paths to ESP IDF files used by the parser
fn esp_idf_paths(esp_idf_path: &Path) -> parser::Result<EspIdfPaths> {
    let mut paths = EspIdfPaths::default();

    paths.base = esp_idf_path.canonicalize()?;
    paths.component = paths.base.join("components");
    paths.soc_component = paths.component.join("soc");
    paths.soc_esp32_include = paths.soc_component.join("esp32/include/soc/soc.h");

    Ok(paths)
}

/// Parses ESP IDF files located at `esp_idf_path` into an `EspIdf` struct
pub fn read_esp_idf(esp_idf_path: &Path) -> parser::Result<EspIdf> {
    let mut esp_idf = EspIdf::default();
    esp_idf.paths = esp_idf_paths(esp_idf_path)?;
    esp_idf.soc = parser::parse_text_file(&esp_idf.paths.soc_esp32_include)?;

    Ok(esp_idf)
}
