use std::{
    io, process,
    sync::{Arc, Mutex},
};

use colored::Colorize;
use log::debug;
use regex::Regex;
use threadpool::ThreadPool;

use crate::{cli, commands, config, constants};

enum ContentType {
    Toml(config::schemas::ConfigFile),
    Plain(String),
}

pub fn process_without_subcommand(cli: &cli::schemas::Cli) {
    let contents: ContentType = match cli.toml {
        true => ContentType::Toml(get_toml_config(
            config::PATH_TO_CONFIG_DIR,
            config::TOML_CONFIG_FILE,
        )),
        false => {
            let contents =
                get_config_file_contents(config::PATH_TO_CONFIG_DIR, config::CONFIG_FILE_NAME);
            match contents {
                Ok(file) => ContentType::Plain(file),
                Err(e) => {
                    eprintln!("Error while parsing config file: {e:?}");
                    process::exit(1);
                }
            }
        }
    };

    match contents {
        ContentType::Toml(file) => handle_toml_file(file),
        ContentType::Plain(string) => handle_plain_file(string),
    };
}

fn handle_toml_file(file: config::schemas::ConfigFile) {
    let paths: Vec<String> = file.paths.iter().map(|p| p.path.clone()).collect();

    let outputs = get_status_from_paths(paths);
    let (clean, dirty) = parse_outputs(outputs);

    println!("{}", "The following repos are clean:".green());
    for output in clean {
        println!("\t{}", output.path);
    }
    println!();

    println!("{}", "The following repos are dirty:".red());
    for output in dirty {
        println!("\t{}", output.path);
    }
}

fn handle_plain_file(contents: String) {
    let paths = config::generate_list_of_paths(contents);

    let outputs = get_status_from_paths(paths);
    let (clean, dirty) = parse_outputs(outputs);

    println!("{}", "The following repos are clean:".green());
    for output in clean {
        println!("\t{}", output.path);
    }
    println!();

    println!("{}", "The following repos are dirty:".red());
    for output in dirty {
        println!("\t{}", output.path);
    }
}

fn get_config_file_contents(path: &str, filename: &str) -> io::Result<String> {
    let mut path = config::parse_path_with_tilde(path).unwrap();
    path.push_str(filename);
    let contents = config::read_config_file(&path).unwrap();
    debug!("Contents:\t{:?}", contents);
    Ok(contents)
}

fn get_toml_config(path: &str, filename: &str) -> config::schemas::ConfigFile {
    let mut path = config::parse_path_with_tilde(path).unwrap();
    path.push_str(filename);
    let contents = config::read_toml_file(&path).unwrap();
    debug!("Contents:\n{:#?}", contents);
    contents
}

fn get_status_from_paths(paths: Vec<String>) -> Vec<commands::Outputs> {
    let mutex = create_threads(paths);

    let outputs = mutex.lock().unwrap().to_vec();

    outputs
}

fn create_threads(paths: Vec<String>) -> Arc<Mutex<Vec<commands::Outputs>>> {
    let pool = ThreadPool::new(constants::NUM_OF_WORKERS);
    let mutex = Arc::new(Mutex::new(vec![]));

    for path in paths {
        let mutex = mutex.clone();
        pool.execute(move || commands::git_status(path, mutex));
    }

    pool.join();

    mutex
}

fn parse_outputs(
    outputs: Vec<commands::Outputs>,
) -> (Vec<commands::Outputs>, Vec<commands::Outputs>) {
    let clean_working_tree_re = Regex::new(constants::PATTERN).unwrap();

    let mut clean_repos: Vec<commands::Outputs> = Vec::new();
    let mut dirty_repos: Vec<commands::Outputs> = Vec::new();

    for output in outputs {
        debug!("Looking at repo: {}", output.path);
        debug!("stdout:\n{}", output.stdout);
        if output.stderr.len() > 0 {
            debug!("stderr:\n{}", output.stderr.len());
        }

        if clean_working_tree_re.is_match(&output.stdout) {
            clean_repos.push(output);
        } else {
            debug!("Path of {} is dirty", output.path);
            dirty_repos.push(output);
        }
    }

    (clean_repos, dirty_repos)
}
