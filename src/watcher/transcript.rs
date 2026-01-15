use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

/// Watches a transcript file for changes (indicating compaction is complete)
#[derive(Debug)]
pub struct TranscriptWatcher {
    path: PathBuf,
    initial_size: u64,
    initial_mtime: SystemTime,
}

impl TranscriptWatcher {
    /// Create a new watcher for the given path
    /// Records initial size and mtime to detect changes
    pub fn new(path: PathBuf) -> std::io::Result<Self> {
        let metadata = fs::metadata(&path)?;
        Ok(Self {
            path,
            initial_size: metadata.len(),
            initial_mtime: metadata.modified()?,
        })
    }

    /// Check if the file has changed since we started watching
    /// Detects any size change (compaction typically shrinks the file)
    pub fn file_changed(&self) -> std::io::Result<bool> {
        let metadata = fs::metadata(&self.path)?;
        let current_size = metadata.len();
        let current_mtime = metadata.modified()?;

        Ok(current_size != self.initial_size || current_mtime > self.initial_mtime)
    }
}
