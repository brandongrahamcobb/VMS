#[cfg(test)]
mod tests {

    use config::error::ConfigError;
    use config::settings;

    // #[test]
    fn test_wz_directory() -> Result<(), ConfigError> {
        unsafe {
            std::env::set_var("WZ_DIRECTORY", "data/");
        }
        let wz_directory: String = settings::get_wz_path()?;
        assert_eq!(String::from("data/"), wz_directory);
        Ok(())
    }
}
