use std::path::PathBuf;
use fancy_regex::Regex;

fn fidwe_inner(dir: PathBuf, ext: &str, buf: &mut Vec<String>) -> anyhow::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.metadata()?.is_dir() {
            fidwe_inner(path, ext, buf)?;
        } else {
            if path.extension().map_or(false, |fext| fext == ext) {
                if let Some(fpath) = path.to_str() {
                    buf.push(fpath.to_string());
                }
            }
        }
    }
    Ok(())
}

pub fn files_in_dir_with_ext(dir: impl Into<String>, ext: impl Into<String>) -> anyhow::Result<Vec<String>> {
    let dir = dir.into();
    let ext = ext.into();
    let mut buf = Vec::new();
    fidwe_inner(PathBuf::from(&dir), &ext, &mut buf)?;
    Ok(buf)
}

pub fn jbeam_to_json(data: impl Into<String>) -> String {
    let data = data.into();
    // Add commas to each line
    let data: String = data.lines().into_iter().map(|line| if line != "" {
        format!("{line},\n")
    } else {
        format!("{line}\n")
    }).collect();

    // Strip trailing commas
    let data = remove_trailing_commas(data);

    data
}

const TRAILING_COMMAS_REGEX: &'static str = r#",((?!\s*?[\{\[\"\'\w])|(?<=[\{\[],))"#;

pub fn remove_trailing_commas(data: impl Into<String>) -> String {
    let data = data.into();
    let re = Regex::new(TRAILING_COMMAS_REGEX).unwrap();
    re.replace_all(&data, "").to_string()
}
