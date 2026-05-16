#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::metadata;
    use crate::metadata::error::MetadataError;

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
            // "Map.wz",
            // "Morph.wz",
            // "Quest.wz",
            "Skill.wz",
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
            match metadata::service::wz_debug_dir(&filename, "") {
                Ok(json) => {
                    // let item_key = format!("{:08}", wz);
                    println!("{}", serde_json::to_string_pretty(&json).unwrap());
                    // if let Some(info) = &json["info"].as_str() {
                    //     dbg!(&info);
                    // }
                    // if let Some(islot) = json["info"]["islot"].as_str() {
                    //     islots.insert(wz, islot.to_string());
                    // }
                }
                Err(_) => (),
            }
        }
        Ok(())
    }
}
