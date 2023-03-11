const DODS_CONFIG_LOCATION_WIN32: &'static str =
    r#"C:\Program Files (x86)\Steam\steamapps\common\Day of Defeat Source\dod\cfg"#;
const DODS_CONFIG_LOCATION_LINUX: &'static str =
    r#"C:\Program Files (x86)\Steam\steamapps\common\Day of Defeat Source\dod\cfg"#;

fn get_cfg_path() -> &'static str {
    return match std::env::consts::OS {
        "windows" => DODS_CONFIG_LOCATION_WIN32,
        "linux" => DODS_CONFIG_LOCATION_LINUX,
        _ => "",
    };
}
