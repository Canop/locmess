use {
    crate::*,
    anyhow::*,
    git2::Repository,
    std::{
        cmp::Reverse,
        collections::HashMap,
        ffi::OsStr,
        path::PathBuf,
    },
};

pub struct FileFinder {
    root: PathBuf,
}

impl FileFinder {
    pub fn new(
        root: PathBuf,
    ) -> Self {
        Self { root }
    }
    pub fn source_files(self, mandatory_ext: Option<&str>) -> Result<Vec<PathBuf>> {
        if !self.root.is_dir() {
            return Ok(vec![self.root]);
        }
        let mut files = Vec::new();
        let mut dirs = Vec::new();
        let repo = Repository::discover(&self.root)?;
        dirs.push(self.root);
        while let Some(dir) = dirs.pop() {
            for entry in dir.read_dir()? {
                let path = entry?.path();
                if repo.is_path_ignored(&path)? {
                    continue;
                }
                if path.is_dir() {
                    dirs.push(path);
                } else {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if let Some(mandatory_ext) = mandatory_ext {
                            if ext == mandatory_ext {
                                files.push(path);
                            }
                        } else if !is_known_binary(ext) {
                            files.push(path);
                        }
                    }
                }
            }
        }
        Ok(files)
    }
    pub fn main_type_files(self) -> Result<Vec<PathBuf>> {
        let all_files = self.source_files(None)?;
        let mut map_by_types: HashMap<&OsStr, usize> = HashMap::new();
        for path in &all_files {
            if let Some(ext) = path.extension() {
                *map_by_types.entry(ext).or_default() += 1;
            }
        }
        let mut arr_by_types: Vec<(&OsStr, usize)> = map_by_types
            .drain()
            .collect();
        if arr_by_types.is_empty() {
            bail!("no suitable source file found");
        }
        arr_by_types.sort_by_key(|(_, count)| Reverse(*count));
        let chosen_ext = arr_by_types[0].0;
        print!("Files per type:");
        for e in &arr_by_types {
            print!(" {}:{} ", e.0.to_string_lossy(), e.1);
        }
        println!();
        println!(
            "-> choosing {} (use `-t` to choose another file type)",
            chosen_ext.to_string_lossy(),
        );
        Ok(
            all_files
                .iter()
                .filter(|path| path.extension()==Some(chosen_ext))
                .cloned()
                .collect()
        )
    }
}

