use std::iter::FromIterator;
use walkdir::DirEntry;
use std::path::Path;
use std::collections::HashSet;
use std::str::FromStr;
use neon::prelude::*;
use std::fs;
use std::process;
use std::path;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use walkdir::WalkDir;


const INCRE_CONFIG_PATH: &str = "incre.config.json";
const CACHE_PATH: &str = "incre";
const CACHE_FILE: &str = "cache.json";

mod cache;

#[derive(Serialize, Deserialize)]
struct Config {
    include: Vec<String>,
    // exclude: Vec<String>
}
impl Config {
    fn get_config() -> Option<Config> {
        let exists = fs::metadata(INCRE_CONFIG_PATH).is_ok();
        if !exists {
            return None;
        }
        let file = fs::read_to_string(INCRE_CONFIG_PATH).unwrap();
        Some(serde_json::from_str(&file).unwrap())
    }
}

fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    println!("Loading Config");
    let configOptional = Config::get_config();
    if configOptional.is_none() {
        eprintln!("Unable to find config {}", INCRE_CONFIG_PATH);
        std::process::exit(2)
    }
    let config = configOptional.unwrap();
    println!("Config Loaded");
    let cache: cache::CachedEntry = {
        cache::CachedEntry::from_file(Path::new(CACHE_PATH).join(CACHE_FILE).to_str().unwrap()).unwrap_or(cache::CachedEntry::new(HashSet::new()))
    };
    let mut changed = false;
    if !cache.is_empty() {
        println!("Cache is not empty"); 
        match cache.was_changed() {
            Ok(result) => { 
                changed = result;
            }
            Err(_) => {}
        }
    } else {
        println!("Cache is empty");
        changed = true;
    }
    if changed {
        println!("Changes Detected"); 
        let files = config
                    .include
                    .into_iter()
                    .flat_map(|d| walk(&d))
                    .map(|f| cache::CachedFile::from_filename(&f))
                    .filter(|f| f.is_ok())
                    .map(|f| f.unwrap());
        let newCache = cache::CachedEntry::new(HashSet::from_iter(files));
        newCache.write(CACHE_FILE, CACHE_PATH).unwrap();
    }
    Ok(cx.undefined())

}

fn walk (dir: &str) -> Vec<String> {
    WalkDir::new(dir)
    .into_iter()
    .filter_map(|e| e.ok())
    .map(|f| String::from(f.path().to_str().unwrap_or("")))
    .filter(|p| !p.is_empty() )
    .collect()
}

register_module!(mut cx, {
    cx.export_function("init", init)
});
