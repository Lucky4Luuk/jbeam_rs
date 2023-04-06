use thiserror::Error;
use serde_json::Value;

#[derive(Debug, Error)]
pub enum JbeamSlotError {
    #[error("Slot not found!")]
    SlotNotFound,
}

#[derive(Serialize, Deserialize)]
pub struct JbeamSlot(Vec<String>);

impl std::ops::Deref for JbeamSlot {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct JbeamSlotHeader(Vec<String>);

impl std::ops::Deref for JbeamSlotHeader {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct JbeamSlots(Vec<Value>);

impl JbeamSlots {
    /// The first item in the list should always be a JbeamSlotHeader
    pub fn get_header(&self) -> anyhow::Result<JbeamSlotHeader> {
        let header_value = self.0.get(0).ok_or(JbeamSlotError::SlotNotFound)?;
        Ok(serde_json::from_value(header_value.clone())?)
    }

    pub fn iter_slots(&self) -> impl Iterator<Item = anyhow::Result<JbeamSlot>> + '_ {
        self.0[1..].iter().map(|slot| to_slot(slot.clone()))
    }
}

fn to_slot(slot: Value) -> anyhow::Result<JbeamSlot> {
    Ok(serde_json::from_value(slot)?)
}
