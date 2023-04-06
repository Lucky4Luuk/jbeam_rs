use thiserror::Error;
use crate::jbeam::*;

#[derive(Debug, Error)]
pub enum VehiclePartError {
    #[error("Slot header missing value `type`!")]
    SlotHeaderMissingType,
    #[error("Slot header missing value `default`!")]
    SlotHeaderMissingDefault,
    #[error("Slot missing data!")]
    SlotMissingData,
}

#[derive(Debug, Clone)]
pub struct VehicleSlotEntry {
    pub ty: String,
    pub default_part: String,
}

#[derive(Debug, Clone)]
pub struct VehiclePart {
    path: String,
    pub name: String,

    pub slot_type: String,
    pub slots: Vec<VehicleSlotEntry>,
}

impl VehiclePart {
    pub fn new(path: impl Into<String>, name: impl Into<String>, jbeam_part: &JbeamPart) -> anyhow::Result<Self> {
        let mut slots = Vec::new();
        if let Some(jbeam_slots) = &jbeam_part.slots {
            let header = jbeam_slots.get_header()?;
            let (ty_slot, _) = header.iter().enumerate().find(|(_, v)| v.as_str() == "type").ok_or(VehiclePartError::SlotHeaderMissingType)?;
            let (default_part, _) = header.iter().enumerate().find(|(_, v)| v.as_str() == "default").ok_or(VehiclePartError::SlotHeaderMissingDefault)?;
            for jbeam_slot in jbeam_slots.iter_slots() {
                let jbeam_slot = jbeam_slot?;
                slots.push(VehicleSlotEntry {
                    ty: jbeam_slot.get(ty_slot).ok_or(VehiclePartError::SlotMissingData)?.to_string(),
                    default_part: jbeam_slot.get(default_part).ok_or(VehiclePartError::SlotMissingData)?.to_string(),
                });
            }
        }

        Ok(Self {
            path: path.into(),
            name: name.into(),

            slot_type: jbeam_part.slot_type.clone(),
            slots,
        })
    }
}

#[derive(Debug)]
pub struct VehiclePartTreeEntry {
    pub part: VehiclePart,
    pub children: Vec<VehiclePart>,
}

impl VehiclePartTreeEntry {
    pub fn from_part(part: VehiclePart, mut parts: Vec<VehiclePart>) -> Self {
        let mut children = Vec::new();
        for slot_entry in &part.slots {
            while let Some((part_idx, _)) = parts.iter().enumerate().find(|(_i, part)| part.slot_type == slot_entry.ty) {
                let child_part = parts.remove(part_idx);
                children.push(child_part);
            }
        }
        Self {
            part,
            children,
        }
    }
}

#[derive(Debug)]
pub struct VehiclePartTree {
    pub root: VehiclePartTreeEntry,
}

impl VehiclePartTree {
    pub fn from_parts(mut parts: Vec<VehiclePart>) -> Self {
        let (root_part_idx, _) = parts.iter().enumerate().find(|(_i, part)| part.slot_type == "main").unwrap();
        let root_part = parts.remove(root_part_idx);
        let root = VehiclePartTreeEntry::from_part(root_part, parts);
        Self {
            root,
        }
    }
}
