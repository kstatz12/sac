use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
struct Game {
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
            _ => unreachable!()
        };
    }
}

pub struct Config {
    games: Vec<Game>,
    pub file_contents: Vec<String>
}

impl Config {
    pub fn new(path: PathBuf) -> Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let games: Vec<Game> = serde_json::from_str(&contents).unwrap();
        Config { games, file_contents: Vec::new() }
    }

    pub fn load(&self, game: String) {
        let game = self.games.iter().filter(|&x| {
          x.name == game
        }).collect::<Vec<&Game>>().get(0).unwrap();


    }
}
