use std::fs;
use crate::parser::Result;
use crate::parser::esp_idf;

#[derive(Default, Debug, Clone)]
pub struct Soc {

}

pub fn read_soc(paths: &esp_idf::EspIdfPaths) -> Result<Soc> {
    let esp32_soc_h = fs::read_to_string(&paths.soc_esp32_include)?;

    Ok(Soc {})
}
