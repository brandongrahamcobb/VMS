use crate::config::settings;
use crate::models::error::ModelError;
use crate::models::wz::error::WzError;
use serde_json;
use shroom_img::value::{Object, Value};
use shroom_wz::reader::WzReader;
use shroom_wz::{WzContext, WzDir, WzDirEntry, try_detect_file_versions};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn load_wz_reader(filename: String) -> Result<WzReader<BufReader<File>>, ModelError> {
    let path: String = settings::get_wz_path()?;
    let base = std::path::Path::new(&path).join("Base.wz");
    let file = std::path::Path::new(&path).join(filename);
    let version = try_detect_file_versions(&base)
        .map_err(WzError::BinRwError)?
        .into_iter()
        .next()
        .ok_or(WzError::NoVersion)
        .map_err(ModelError::from)?;
    let wz_ctx = WzContext::global(version).shared();
    let reader = WzReader::open(file, wz_ctx).map_err(WzError::AnyHowError)?;
    Ok(reader)
}

fn build_index(
    wz: &mut WzReader<BufReader<File>>,
    dir: &WzDir,
    prefix: String,
    map: &mut HashMap<i32, String>,
) -> Result<(), ModelError> {
    for entry in dir.0.iter() {
        match entry {
            WzDirEntry::Dir(sub) => {
                let name = &sub.name.0;
                let path = if prefix.is_empty() {
                    name.to_string()
                } else {
                    format!("{}/{}", prefix, name)
                };
                let sub_dir = wz.read_dir_node(sub).map_err(WzError::AnyHowError)?;
                build_index(wz, &sub_dir, path, map);
            }
            WzDirEntry::Img(img) => {
                let name = &img.name.0;
                let path = if prefix.is_empty() {
                    name.to_string()
                } else {
                    format!("{}/{}", prefix, name)
                };
                if let Some(id) = name.strip_suffix(".img") {
                    if let Ok(id_num) = id.parse::<i32>() {
                        map.insert(id_num, path);
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

pub struct WzResolver {
    map: HashMap<i32, String>,
}

impl WzResolver {
    pub fn new(wz_reader: &mut WzReader<BufReader<File>>) -> Result<Self, ModelError> {
        let dir = wz_reader
            .read_root_dir()
            .map_err(WzError::AnyHowError)
            .map_err(ModelError::from)?;
        let mut map = HashMap::new();
        build_index(wz_reader, &dir, String::new(), &mut map);
        Ok(Self { map })
    }

    pub fn resolve(&self, id: i32) -> Result<Option<&String>, ModelError> {
        Ok(self.map.get(&id))
    }
}

pub fn get_i32(map: &serde_json::Value, key: &str) -> Option<i32> {
    map.get(key).and_then(|v| v.as_i64().map(|n| n as i32))
}

pub fn get_img_map(id: i32, wz_name: String) -> Result<serde_json::Value, ModelError> {
    let mut wz = load_wz_reader(wz_name)?;
    let resolver = WzResolver::new(&mut wz)?;
    let path = resolver
        .resolve(id)?
        .ok_or(WzError::NotFound(id))
        .map_err(ModelError::from)?;
    let parts: Vec<&str> = path.split('/').collect();
    let root = wz
        .read_root_dir()
        .map_err(WzError::AnyHowError)
        .map_err(ModelError::from)?;
    let mut node = root;
    for part in &parts {
        let entry = node
            .get(part)
            .ok_or(WzError::EntryError)
            .map_err(ModelError::from)?;
        node = match entry {
            WzDirEntry::Dir(d) => wz
                .read_dir_node(d)
                .map_err(WzError::AnyHowError)
                .map_err(ModelError::from)?,
            WzDirEntry::Img(img) => {
                let img_hdr = img;
                let mut img = wz
                    .img_reader(img_hdr)
                    .map_err(WzError::AnyHowError)
                    .map_err(ModelError::from)?;
                let root_obj = Object::from_reader(&mut img)
                    .map_err(WzError::BinRwError)
                    .map_err(ModelError::from)?;
                let root_obj = root_obj.as_property().unwrap();
                if let Some(Value::Object(obj)) = root_obj.get("info") {
                    return Ok(obj.to_json_value());
                }
                return Err(ModelError::from(WzError::ObjectError));
            }
            _ => return Err(ModelError::from(WzError::EntryError)),
        };
    }
    Err(ModelError::from(WzError::PartError))
}
