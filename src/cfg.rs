use std::{fs, path::PathBuf, io::{BufReader, Read}};
use std::fs::File;
use crypto::digest::Digest;
use crypto::sha2::Sha256;

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

pub struct Config {
    pub content: String,
    pub hash: String,
    pub fullname: String,
}

impl Config {
    fn new(content: &str, hash: &str, fullname: &str) -> Self {
        Config {
            content: String::from(content),
            hash: String::from(hash),
            fullname: String::from(fullname),
        }
    }
}

pub struct ConfigFile {
    pub path: PathBuf,
}

impl ConfigFile {
    fn new(filename: &str) -> Self {
        let p: PathBuf = PathBuf::from(get_cfg_path());
        p.push(filename);
        ConfigFile { path: p }
    }

    fn to_config(&self) -> Config {
        let contents = fs::read_to_string(self.path).unwrap();
        let hash = hash(self.path);
        return Config::new(contents.as_str(), hash, self.path.to_str());
}

fn hash<'a>(path: PathBuf) -> &'a str {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut sha256 = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        sha256.input(&buffer[..bytes_read]);
    }
    return sha256.result_str();
}
