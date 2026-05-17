#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::metadata;
    use crate::metadata::error::MetadataError;
    use crate::models::item;

    #[test]
    fn test_islots() -> Result<(), MetadataError> {
        unsafe {
            // std::env::set_var("WZ_DIRECTORY", concat!(env!("CARGO_MANIFEST_DIR"), "/data"));
            std::env::set_var("WZ_DIRECTORY", "./data");
        }
        let filenames: Vec<&str> = vec![
            // "Base.wz",
            // "Effect.wz",
            // "Item.wz",
            "Map.wz",
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
            let wz = 10000;
            match metadata::service::wz_to_img(wz, &filename) {
                Ok(json) => {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&json["info"]["returnMap"]).unwrap()
                    );
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
