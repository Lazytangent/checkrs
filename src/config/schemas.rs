use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigFile {
    pub paths: Vec<Path>,
}

#[derive(Deserialize, Debug)]
pub struct Path {
    pub name: String,
    pub path: String,
}
