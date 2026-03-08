use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct AssetInfo {
    pub name : String,
    pub author : String,
    pub source : String,
    pub license : String,
    pub ty : String,
    pub custom_attribution : Option<String>,
}
