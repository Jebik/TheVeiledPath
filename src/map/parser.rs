use std::path::PathBuf;
use std::{fs, error::Error, str::FromStr};

use crate::map::json_types::{Dimension, MapData};

impl FromStr for Dimension {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" => Ok(Dimension::Light),
            "dark" => Ok(Dimension::Dark),
            _ => Err(()),
        }
    }
}
pub enum MapFile {
    Level,
    Tuto,
}

pub enum MapSource {
    FilePath(PathBuf),
    FileContent(MapFile),
}

pub fn parse_map(map_source: MapSource) -> Result<MapData, Box<dyn Error>> {
    let file = match map_source {
        MapSource::FilePath(path) => fs::read_to_string(path)?,
        MapSource::FileContent(map_file) => match map_file {
            MapFile::Level => include_str!("data/level.json").to_string(),
            MapFile::Tuto => include_str!("data/tuto.json").to_string(),
        },
    };

    let data: MapData = serde_json::from_str(&file)?;
    
    // Check if the size is a multiple of 16
    if data.size % 16 != 0 {
        return Err("Map size must be a multiple of 16".into());
    }
    Ok(data)
}



