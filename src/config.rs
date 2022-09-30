pub mod schemas;

use std::{
    env,
    error::Error,
    fs::{DirBuilder, File},
    io::Read,
    path::Path,
    process,
};

use log::{debug, error};

pub const PATH_TO_CONFIG_DIR: &str = "~/.config/checkrs/";
pub const CONFIG_FILE_NAME: &str = "config";
pub const TOML_CONFIG_FILE: &str = "config.toml";

pub fn parse_path_with_tilde(path: &str) -> Result<String, String> {
    let path = String::from(path);
    debug!("Path starts as: {}", path);

    let home = match env::var("HOME") {
        Ok(path) => path,
        Err(e) => return Err(format!("Error while parsing HOME env var: {}", e)),
    };

    let path = path.replace("~", &home);
    debug!("Path is now: {}", path);

    Ok(path)
}

pub fn read_config_file(path: &str) -> Result<String, Box<dyn Error>> {
    if !check_config_dir() {
        println!("Directory not found.");
        println!("Attempting to create one.");
        // TODO: Remove extra call for same path
        let path = match parse_path_with_tilde(PATH_TO_CONFIG_DIR) {
            Ok(val) => val,
            Err(e) => {
                error!("Error while parsing path");
                eprintln!("{e:#?}");
                return Err(Box::<dyn Error>::from(e));
            }
        };

        match DirBuilder::new().create(path) {
            Ok(_) => println!("Successfully created directory"),
            Err(e) => return Err(Box::<dyn Error>::from(e)),
        }
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            // TODO: Should there be logic to only create a new when certain kind(s) of errors
            // appear?
            eprintln!("File not found: {}", e);
            debug!("Creating a new file");

            // TODO: Prompt user for permission to create a new config file.
            match File::create(path) {
                Ok(file) => {
                    println!("Successfully created file");
                    file
                }
                Err(e) => return Err(Box::<dyn Error>::from(e)),
            }
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn generate_list_of_paths(contents: String) -> Vec<String> {
    let mut paths_to_check: Vec<String> = Vec::new();

    for line in contents.lines() {
        let path = match parse_path_with_tilde(line) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error while parsing path: {e:?}");
                process::exit(1);
            }
        };
        paths_to_check.push(path);
    }

    debug!("Paths: {:?}", paths_to_check);
    paths_to_check
}

fn check_config_dir() -> bool {
    let path = parse_path_with_tilde(PATH_TO_CONFIG_DIR).unwrap();

    Path::new(&path).is_dir()
}

pub fn read_toml_file(path: &str) -> Result<schemas::ConfigFile, Box<dyn Error>> {
    if !check_config_dir() {
        println!("Directory not found.");
        println!("Attempting to create one.");
        // TODO: Remove extra call for same path
        let path = match parse_path_with_tilde(PATH_TO_CONFIG_DIR) {
            Ok(val) => val,
            Err(e) => {
                error!("Error while parsing path");
                eprintln!("{e:#?}");
                return Err(Box::<dyn Error>::from(e));
            }
        };

        match DirBuilder::new().create(path) {
            Ok(_) => println!("Successfully created directory"),
            Err(e) => return Err(Box::<dyn Error>::from(e)),
        }
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            // TODO: Should there be logic to only create a new when certain kind(s) of errors
            // appear?
            eprintln!("File not found: {}", e);
            debug!("Creating a new file");

            // TODO: Prompt user for permission to create a new config file.
            match File::create(path) {
                Ok(file) => {
                    println!("Successfully created file");
                    file
                }
                Err(e) => return Err(Box::<dyn Error>::from(e)),
            }
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let contents: schemas::ConfigFile = toml::from_str(&contents).unwrap();

    Ok(contents)
}
