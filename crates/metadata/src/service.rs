/* metadata/src/service.rs
 * The purpose of this module is to provide assisting functions and implementations for Wz data.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::error::MetadataServiceError;
use config::settings;
use serde_json;
use shroom_img::value::Object;
use shroom_wz::reader::WzReader;
use shroom_wz::{WzContext, WzDir, WzDirEntry, try_detect_file_versions};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

fn load_wz_reader(filename: &str) -> Result<WzReader<BufReader<File>>, MetadataServiceError> {
    let path: String = settings::get_wz_path()?;
    let base = std::path::Path::new(&path).join("Base.wz");
    let file = std::path::Path::new(&path).join(filename);
    let version = try_detect_file_versions(&base)
        .map_err(MetadataServiceError::BinRwError)?
        .into_iter()
        .next()
        .ok_or(MetadataServiceError::NoVersion)?;
    let wz_ctx = WzContext::global(version).shared();
    let reader = WzReader::open(&file, wz_ctx).map_err(MetadataServiceError::AnyHowError)?;
    Ok(reader)
}

fn wz_resolve_img<F, T>(wz: i32, wz_name: &str, f: F) -> Result<T, MetadataServiceError>
where
    F: FnOnce(&Object, &[&str], &str) -> Result<T, MetadataServiceError>,
{
    let mut wz_reader = load_wz_reader(wz_name)?;
    let resolver = WzResolver::new(&mut wz_reader)?;
    let path = resolver.resolve(wz)?;
    let parts: Vec<&str> = path.split('/').collect();
    let root = wz_reader
        .read_root_dir()
        .map_err(MetadataServiceError::AnyHowError)?;
    let mut node = root;
    for (i, part) in parts.iter().enumerate() {
        let entry = node.get(part).ok_or(MetadataServiceError::EntryError)?;
        match entry {
            WzDirEntry::Dir(dir) => {
                node = wz_reader
                    .read_dir_node(&dir)
                    .map_err(MetadataServiceError::AnyHowError)?;
            }
            WzDirEntry::Img(img) => {
                let mut img_reader = wz_reader
                    .img_reader(&img)
                    .map_err(MetadataServiceError::AnyHowError)?;
                let obj = Object::from_reader(&mut img_reader)
                    .map_err(MetadataServiceError::BinRwError)?;
                return f(&obj, &parts[..=i], wz_name);
            }
            _ => return Err(MetadataServiceError::EntryError),
        }
    }
    Err(MetadataServiceError::PartError)
}

pub fn wz_to_tree(wz: i32, wz_name: &str) -> Result<serde_json::Value, MetadataServiceError> {
    wz_resolve_img(wz, wz_name, |obj, dir_parts, wz_name| {
        let prop = obj.as_property().ok_or(MetadataServiceError::EntryError)?;
        let keys: serde_json::Value = prop
            .0
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<_>>()
            .into();
        let full = dir_parts
            .iter()
            .rev()
            .fold(keys, |acc, segment| serde_json::json!({ *segment: acc }));
        Ok(serde_json::json!({ wz_name: full }))
    })
}

pub fn wz_to_img(wz: i32, wz_name: &str) -> Result<serde_json::Value, MetadataServiceError> {
    wz_resolve_img(wz, wz_name, |obj, _dir_parts, _wz_name| {
        Ok(obj.to_json_value())
    })
}

pub fn wz_debug_dir(
    wz_name: &str,
    dir_path: &str,
) -> Result<serde_json::Value, MetadataServiceError> {
    let mut wz_reader = load_wz_reader(wz_name)?;
    let parts: Vec<&str> = dir_path.split('/').filter(|s| !s.is_empty()).collect();
    let root = wz_reader
        .read_root_dir()
        .map_err(MetadataServiceError::AnyHowError)?;
    let mut node = root;
    for part in &parts {
        let entry = node.get(part).ok_or(MetadataServiceError::EntryError)?;
        match entry {
            WzDirEntry::Dir(dir) => {
                node = wz_reader
                    .read_dir_node(dir)
                    .map_err(MetadataServiceError::AnyHowError)?;
            }
            _ => return Err(MetadataServiceError::EntryError),
        }
    }
    let keys: serde_json::Value = node
        .0
        .iter()
        .filter_map(|entry| match entry {
            WzDirEntry::Dir(d) => Some(format!("{}/", d.name.0)),
            WzDirEntry::Img(i) => Some(i.name.0.to_string()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .into();
    Ok(keys)
}

struct WzResolver {
    map: HashMap<i32, String>,
}

impl WzResolver {
    pub fn new(wz_reader: &mut WzReader<BufReader<File>>) -> Result<Self, MetadataServiceError> {
        let mut map = HashMap::new();
        let root = wz_reader
            .read_root_dir()
            .map_err(MetadataServiceError::AnyHowError)?;
        Self::scan(wz_reader, &root, String::new(), &mut map)?;
        Ok(Self { map })
    }

    fn scan(
        wz_reader: &mut WzReader<BufReader<File>>,
        dir: &WzDir,
        prefix: String,
        map: &mut HashMap<i32, String>,
    ) -> Result<(), MetadataServiceError> {
        for entry in dir.0.iter() {
            match entry {
                WzDirEntry::Dir(sub) => {
                    let name = &sub.name.0;
                    let path = if prefix.is_empty() {
                        name.to_string()
                    } else {
                        format!("{}/{}", prefix, name)
                    };
                    let sub_dir = wz_reader
                        .read_dir_node(&sub)
                        .map_err(MetadataServiceError::AnyHowError)?;
                    Self::scan(wz_reader, &sub_dir, path, map)?;
                }
                WzDirEntry::Img(img) => {
                    let name = &img.name.0;
                    let path = if prefix.is_empty() {
                        name.to_string()
                    } else {
                        format!("{}/{}", prefix, name)
                    };
                    if let Some(id) = name.strip_suffix(".img")
                        && let Ok(id_num) = id.parse::<i32>()
                    {
                        map.insert(id_num, path);
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn resolve(&self, id: i32) -> Result<String, MetadataServiceError> {
        self.map
            .get(&id)
            .cloned()
            .ok_or(MetadataServiceError::NotFound(id))
    }
}
