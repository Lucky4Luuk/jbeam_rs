use super::*;

#[derive(Serialize, Deserialize)]
pub struct JbeamVehicleInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Brand")]
    pub brand: String,
    #[serde(rename = "Author")]
    pub author: Option<String>,
}

pub struct JbeamVehicle {
    pub info: JbeamVehicleInfo,

    pub jbeam_files: HashMap<String, JbeamFile>,
}

impl JbeamVehicle {
    pub fn from_folder(path: impl Into<String>) -> anyhow::Result<Self> {
        let path = path.into();
        let info_raw = jbeam_to_json(std::fs::read_to_string(format!("{path}/info.json"))?);
        let info = serde_json::from_str(&info_raw)?;

        let mut jbeam_files = HashMap::new();
        for jbeam_path in files_in_dir_with_ext(path, "jbeam")? {
            println!("Loading file `{jbeam_path}`...");
            let jbeam_file = JbeamFile::from_path(&jbeam_path)?;
            jbeam_files.insert(jbeam_path, jbeam_file);
        }

        Ok(Self {
            info: info,

            jbeam_files: jbeam_files,
        })
    }
}
