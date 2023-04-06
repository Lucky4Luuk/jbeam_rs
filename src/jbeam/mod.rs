use std::collections::HashMap;
use crate::util::*;

mod vehicle;
mod slots;
mod nodes;

pub use vehicle::*;
pub use slots::*;
pub use nodes::*;

#[derive(Serialize, Deserialize)]
pub struct JbeamPartInfo {
    authors: Option<String>,
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JbeamPart {
    pub information: JbeamPartInfo,
    #[serde(rename = "slotType")]
    pub slot_type: String,
    pub slots: Option<JbeamSlots>,
    pub nodes: Option<JbeamNodes>,
}

#[derive(Serialize, Deserialize)]
pub struct JbeamFile(HashMap<String, JbeamPart>);

impl std::ops::Deref for JbeamFile {
    type Target = HashMap<String, JbeamPart>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for JbeamFile {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl JbeamFile {
    pub fn from_path(path: impl Into<String>) -> anyhow::Result<Self> {
        let data = jbeam_to_json(std::fs::read_to_string(path.into())?);
        Ok(serde_json::from_str(&data)?)
    }
}
