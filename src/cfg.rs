use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use crate::common::Startable;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Game {
    windows_path: PathBuf,
    linux_path: PathBuf,
    macos_path: PathBuf,
    configuration_ext: String,
    name: String,
}

impl Game {
    fn resolve_path(&self) -> &PathBuf {
        return match std::env::consts::OS {
            "windows" => &self.windows_path,
            "linux" => &self.linux_path,
            "macos" => &self.macos_path,
            _ => unreachable!(),
        };
    }
}

struct FileContents {
    content: String,
    hash: String,
}

pub struct ConfigService {
    game: Game,
    pub file_contents: Vec<String>,
}

impl ConfigService {
    pub fn new(path: PathBuf, game_name: String) -> Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let games: Vec<Game> = serde_json::from_str(&contents).unwrap();
        let game = games
            .iter()
            .filter(|&x| x.name == game_name)
            .cloned()
            .collect::<Vec<Game>>()
            .get(0)
            .expect("Could Not Find Game")
            .clone();

        ConfigService {
            game,
            file_contents: Vec::new(),
        }
    }

    fn collect(&self) {

    }
}

impl Startable for ConfigService {
    fn start(&mut self) {}
}
