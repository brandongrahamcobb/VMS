#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use entity::item;
    use metadata;
    use metadata::error::MetadataServiceError;

    #[test]
    fn test_islots() -> Result<(), MetadataServiceError> {
        unsafe {
            // std::env::set_var("WZ_DIRECTORY", concat!(env!("CARGO_MANIFEST_DIR"), "/data"));
            std::env::set_var("WZ_DIRECTORY", "../data");
        }
        let filenames: Vec<&str> = vec![
            // "Base.wz",
            // "Effect.wz",
            "Item.wz",
            // "Map.wz",
            // "Morph.wz",
            // "Quest.wz",
            // "Skill.wz",
            // "String.wz",
            // "UI.wz",
            // "Character.wz",
            // "Etc.wz",
            // "List.wz",
            // "Mob.wz",
            // "Npc.wz",
            // "Reactor.wz",
            // "Sound.wz",
            // "TamingMob.wz",
        ];
        for filename in filenames {
            // for wz in (..=5200000).step_by(1) {
            //     let wz_cat = wz / 10000;
            // match metadata::service::wz_debug_dir(filename, "Cash") {
            //
            //10000
            let wz: i32 = 4000007;

            let wz_mod = wz.div_euclid(10000);
            match metadata::service::wz_to_img(wz_mod, &filename) {
                Ok(json) => {
                    let padded_wz: String = format!("{:08}", wz);
                    match json.get(padded_wz) {
                        Some(item) => println!("{}", serde_json::to_string_pretty(&item).unwrap()),
                        None => {
                            dbg!("fail");
                            ()
                        }
                    }
                    //
                    // println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    // if let Some(json) = json.as_object() {
                    //     println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    //     if let Some(islot) = json[&wz.to_string()]["info"]["islot"].as_str() {
                    //         println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    //     }
                    // }
                    // let id = json["life"]["0"]["id"].as_str().unwrap().parse::<i32>()?;
                    // let filename = "Mob.wz";
                    // match metadata::service::wz_to_img(id, &filename) {
                    //     Ok(json) => {
                    //         // println!("{}", serde_json::to_string_pretty(&json["life"]).unwrap());
                    //         println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    //     }
                    //     Err(_) => {}
                    // }
                    // if let Some(cat) = json[filename]
                    //     .as_object()
                    //     .and_then(|o| o.keys().next())
                    //     .map(|k| k.as_str())
                    // {
                    //     dbg!(cat);
                    // }
                    // println!("{}", serde_json::to_string_pretty(&json["info"][).unwrap());
                    // println!("{}", serde_json::to_string_pretty(&json["info"][).unwrap());
                    // let filename: String = String::from("Mob.wz");
                    // if let Some(life) = json["life"].as_object() {
                    //     for (key, value) in life {
                    //         println!("{}", serde_json::to_string_pretty(&value).unwrap());
                    //         if let Some(m) = value["type"].as_str() {
                    //             if m == "m" {
                    //                 let id: i32 =
                    //                     value["id"].as_str().unwrap().parse::<i32>().unwrap();
                    //                 let json = metadata::service::wz_to_img(id, &filename)?;
                    //                 // println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    //             }
                    //         }
                    //     }
                    // }
                    //
                    // let item_key = format!("{:08}", wz);
                    // if let Some(info) = &json["info"].as_str() {
                    //     dbg!(&info);
                    // }
                    // if let Some(islot) = json["info"]["islot"].as_str() {
                    //     islots.insert(wz, islot.to_string());
                    // }
                }
                Err(_) => println!("failed"),
            }
        }
        Ok(())
    }
}
