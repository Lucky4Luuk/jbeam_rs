use std::collections::HashMap;
use crate::util::*;
use crate::jbeam::*;

mod vehicle_part;

pub use vehicle_part::*;

#[derive(Debug)]
pub struct VehicleInfo {
    name: String,
    brand: String,
    author: Option<String>,
}

#[derive(Debug)]
pub struct Vehicle {
    info: VehicleInfo,

    part_tree: VehiclePartTree,
}

impl Vehicle {
    pub fn from_folder(path: impl Into<String>) -> anyhow::Result<Self> {
        let path = path.into();
        let jbeam_vehicle = JbeamVehicle::from_folder(path.clone())?;

        let info = VehicleInfo {
            name: jbeam_vehicle.info.name.clone(),
            brand: jbeam_vehicle.info.brand.clone(),
            author: jbeam_vehicle.info.author.clone(),
        };

        let mut parts = Vec::new();
        for (path, file) in &jbeam_vehicle.jbeam_files {
            for (name, part) in file.iter() {
                parts.push(VehiclePart::new(path, name, part)?);
            }
        }

        let part_tree = VehiclePartTree::from_parts(parts);

        Ok(Self {
            info,

            part_tree,
        })
    }

    pub fn name(&self) -> &str {
        &self.info.name
    }
}
