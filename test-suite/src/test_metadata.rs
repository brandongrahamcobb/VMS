#[cfg(test)]
mod tests {

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
                }
                Err(_) => println!("failed"),
            }
        }
        Ok(())
    }
}
