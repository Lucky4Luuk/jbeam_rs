use thiserror::Error;
use serde_json::Value;

#[derive(Debug, Error)]
pub enum JbeamNodeError {
    #[error("Node not found!")]
    NodeNotFound,
}

#[derive(Serialize, Deserialize)]
pub struct JbeamNode(Vec<Value>);

#[derive(Serialize, Deserialize)]
pub struct JbeamNodeHeader(Vec<String>);

#[derive(Serialize, Deserialize)]
pub struct JbeamNodes(Vec<Value>);

impl JbeamNodes {
    /// The first item in the list should always be a JbeamNodeHeader
    pub fn get_header(&self) -> anyhow::Result<JbeamNodeHeader> {
        let header_value = self.0.get(0).ok_or(JbeamNodeError::NodeNotFound)?;
        Ok(serde_json::from_value(header_value.clone())?)
    }
}
