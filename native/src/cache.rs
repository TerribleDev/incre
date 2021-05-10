use std::path::Path;
use std::collections::HashSet;
use std::time::SystemTime;
use std::io;
use std::fs;
use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};
use metrohash::{MetroHash};

#[derive(Serialize, Deserialize)]
pub struct CachedEntry {
    pub files: HashSet<CachedFile>,
}

impl CachedEntry {
    pub fn new(files: HashSet<CachedFile>) -> CachedEntry {
        CachedEntry {
            files,
        }
    }
    // todo Error handle better
    pub fn from_file(path: &str) -> io::Result<CachedEntry> {
        println!("Attempting to read {}", path);
        // Path::new(path).exists()
        fs::metadata(path)?;
        let file = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&file)?)
    }
    pub fn was_changed(&self) -> io::Result<bool> {
        let mut changed = false;
        for cached_file in self.files.iter() {
            if cached_file.was_changed()? {
                changed = true;
                break;
            }
        }
        Ok(changed)
    }
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
    pub fn write(&self, name: &str, cache_dir: &str) -> io::Result<()> {
        let cached_entry_filename = format!("{}", name);
        let cached_directory_path = Path::new(cache_dir);
        if !cached_directory_path.exists() {
            fs::create_dir(cached_directory_path)?
        }
        let cached_entry_path = cached_directory_path.join(cached_entry_filename);
        
        let f = std::io::BufWriter::new(fs::File::create(&cached_entry_path)?);
        serde_json::to_writer(f, self)?;
        Ok(())
    }
}

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct CachedFile {
    path: String,
    size: u64,
    modified: SystemTime,
    hash: u64,
}


impl CachedFile {
    pub fn from_filename(filename: &str) -> io::Result<CachedFile> {
        let file_metadata = fs::metadata(filename)?;
        let size = file_metadata.len();
        let modified = file_metadata.modified()?;
        // TODO: find async way to read file.
        let hash = calculate_hash(&fs::read(filename)?);
        Ok(CachedFile {
            path: filename.to_string(),
            size,
            modified,
            hash,
        })
    }

    pub fn was_changed(&self) -> io::Result<bool> {
        let mut changed = false;
        // println!("Checking if {} has changed", &self.path);
        match fs::metadata(&self.path) {
            Ok(file_metadata) => {
                let size = file_metadata.len();
                let modified = file_metadata.modified()?;
                if self.size != size || self.modified != modified {
                    let hash = calculate_hash(&fs::read(&self.path)?);
                    if self.hash != hash {
                        changed = true;
                    }
                }
            }
            // means previously cached file cannot be read
            // most likely due to the entrypoint of an npm module
            // being changed.
            Err(_) => {
                changed = true;
            }
        }
        Ok(changed)
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = MetroHash::new();
    t.hash(&mut s);
    s.finish()
}

