/* settings.rs
 * The purpose of this module is to access to values in .env.
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
use crate::error::ConfigError;
use cfg::Config;

pub fn get_settings() -> Result<Config, ConfigError> {
    let settings = Config::builder()
        .add_source(cfg::Environment::default())
        .build()
        .unwrap();
    Ok(settings)
}

pub fn get_host() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "host";
    let host = settings
        .get_string(key)
        .map_err(|_| ConfigError::InvalidString(key.to_string()))?;
    Ok(host)
}

pub fn get_bind_address() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "bind_address";
    let addr = settings
        .get_string(key)
        .map_err(|_| ConfigError::InvalidString(key.to_string()))?;
    Ok(addr)
}

pub fn get_routing_address() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "routing_address";
    let addr = settings
        .get_string(key)
        .map_err(|_| ConfigError::InvalidString(key.to_string()))?;
    Ok(addr)
}

pub fn get_item_drop_rate() -> Result<f64, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "item_drop_rate";
    let rate = settings
        .get_float(key)
        .map_err(|_| ConfigError::InvalidFloat(key.to_string()))?;
    Ok(rate)
}

pub fn get_meso_drop_rate() -> Result<f64, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "meso_drop_rate";
    let rate = settings
        .get_float(key)
        .map_err(|_| ConfigError::InvalidFloat(key.to_string()))?;
    Ok(rate)
}

pub fn get_mob_exp_rate() -> Result<f64, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "mob_exp_rate";
    let rate = settings
        .get_float(key)
        .map_err(|_| ConfigError::InvalidFloat(key.to_string()))?;
    Ok(rate)
}

pub fn get_quest_meso_rate() -> Result<f64, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "quest_meso_rate";
    let rate = settings
        .get_float(key)
        .map_err(|_| ConfigError::InvalidFloat(key.to_string()))?;
    Ok(rate)
}

pub fn get_quest_exp_rate() -> Result<f64, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "quest_exp_rate";
    let rate = settings
        .get_float(key)
        .map_err(|_| ConfigError::InvalidFloat(key.to_string()))?;
    Ok(rate)
}

pub fn get_wz_path() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "wz_directory";
    let addr = settings
        .get_string(key)
        .map_err(|_| ConfigError::InvalidString(key.to_string()))?;
    Ok(addr)
}

pub fn get_release_mode() -> Result<bool, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "release_mode";
    let mode = settings
        .get_bool(key)
        .map_err(|_| ConfigError::InvalidString(key.to_string()))?;
    Ok(mode)
}

pub fn get_login_port() -> Result<i16, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "login_port";
    let port = settings
        .get_int(key)
        .map_err(|_| ConfigError::InvalidInt(key.to_string()))?;
    Ok(port as i16)
}

pub fn get_version() -> Result<i16, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "version";
    let version = settings
        .get_int(key)
        .map_err(|_| ConfigError::InvalidInt(key.to_string()))?;
    Ok(version as i16)
}

pub fn get_db_url() -> Result<String, ConfigError> {
    let settings = get_settings()?;
    let db_key: &str = "postgres_database";
    let ip_key: &str = "postgres_host";
    let port_key: &str = "postgres_port";
    let user_key: &str = "postgres_user";
    let pw_key: &str = "postgres_password";
    let db = settings
        .get_string(db_key)
        .map_err(|_| ConfigError::InvalidString(db_key.to_string()))?;
    let ip = settings
        .get_string(ip_key)
        .map_err(|_| ConfigError::InvalidString(ip_key.to_string()))?;
    let port = settings
        .get_int(port_key)
        .map_err(|_| ConfigError::InvalidInt(port_key.to_string()))?;
    let user = settings
        .get_string(user_key)
        .map_err(|_| ConfigError::InvalidString(user_key.to_string()))?;
    let pw = settings
        .get_string(pw_key)
        .map_err(|_| ConfigError::InvalidString(pw_key.to_string()))?;
    Ok(format!("postgres://{}:{}@{}:{}/{}", user, pw, ip, port, db))
}

pub fn get_pin_required() -> Result<bool, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "pin_required";
    let pin_req = settings
        .get_bool(key)
        .map_err(|_| ConfigError::InvalidBool(key.to_string()))?;
    Ok(pin_req)
}

pub fn get_pic_required() -> Result<bool, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "pic_required";
    let pic_req = settings
        .get_bool(key)
        .map_err(|_| ConfigError::InvalidBool(key.to_string()))?;
    Ok(pic_req)
}

pub fn get_gender_required() -> Result<bool, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "gender_required";
    let gender_req = settings
        .get_bool(key)
        .map_err(|_| ConfigError::InvalidBool(key.to_string()))?;
    Ok(gender_req)
}

pub fn get_channel_count() -> Result<u8, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "channel_count";
    let count: u8 = settings
        .get_int(key)?
        .try_into()
        .map_err(ConfigError::IntConversion)?;
    Ok(count)
}

pub fn get_world_count() -> Result<i8, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "world_count";
    let count: i8 = settings
        .get_int(key)?
        .try_into()
        .map_err(ConfigError::IntConversion)?;
    Ok(count)
}

pub fn get_channel_capacity() -> Result<i16, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "channel_capacity";
    let capacity = settings
        .get_int(key)
        .map_err(|_| ConfigError::InvalidInt(key.to_string()))?;
    Ok(capacity as i16)
}

pub fn get_inv_capacity() -> Result<i16, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "inventory_capacity";
    let capacity = settings
        .get_int(key)
        .map_err(|_| ConfigError::InvalidInt(key.to_string()))?;
    Ok(capacity as i16)
}

pub fn get_channel_flag() -> Result<i16, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "channel_flag";
    let flag = settings
        .get_int(key)
        .map_err(|_| ConfigError::InvalidInt(key.to_string()))?;
    Ok(flag as i16)
}

pub fn get_recommended_worlds() -> Result<Vec<String>, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "recommended_worlds";
    let worlds: Vec<String> = settings
        .get_string(key)
        .map_err(|_| ConfigError::InvalidString(key.to_string()))?
        .split(",")
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    Ok(worlds)
}

pub fn get_char_max() -> Result<i16, ConfigError> {
    let settings = get_settings()?;
    let key: &str = "char_max";
    let char_max = settings
        .get_int(key)
        .map_err(|_| ConfigError::InvalidInt(key.to_string()))?;
    Ok(char_max as i16)
}
