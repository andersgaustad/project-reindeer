use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct AssetInfo {
    pub name : String,
    pub author : Option<String>,
    pub source : String,
    pub license : String,
    pub license_source : String,
    pub ty : String,
    pub custom_attribution : Option<String>,
}
