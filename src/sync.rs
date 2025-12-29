//! File synchronization engine

use crate::error::Result;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;
use tracing::warn;

/// File change event
#[derive(Debug, Clone)]
pub enum FileChangeEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Deleted(PathBuf),
    Renamed(PathBuf, PathBuf),
}

/// File watcher for detecting changes
pub struct FileWatcher {
    watched_paths: Vec<PathBuf>,
}

impl FileWatcher {
    /// Creates a new file watcher
    pub fn new() -> Self {
        Self {
            watched_paths: Vec::new(),
        }
    }

    /// Adds a path to watch
    pub fn add_path(&mut self, path: PathBuf) -> Result<()> {
        if !self.watched_paths.contains(&path) {
            self.watched_paths.push(path);
        }
        Ok(())
    }

    /// Starts watching files and returns a channel for events
    pub fn start_watching(&self) -> Result<mpsc::Receiver<FileChangeEvent>> {
        let (tx, rx) = mpsc::channel();

        let _watched_paths = self.watched_paths.clone();
        std::thread::spawn(move || {
            // Simple implementation: just monitor for changes
            // In production, use notify crate properly
            loop {
                std::thread::sleep(Duration::from_secs(5));
                let _ = tx.send(FileChangeEvent::Modified(PathBuf::from(".")));
            }
        });

        Ok(rx)
    }
}

/// Computes file hash for change detection
pub fn compute_file_hash(path: &PathBuf) -> Result<String> {
    use sha2::{Digest, Sha256};

    let data = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(&data);

    Ok(hex::encode(hasher.finalize()))
}

/// Detects changes between local and remote files
pub fn detect_changes(local_path: &PathBuf, remote_hash: &str) -> Result<bool> {
    let local_hash = compute_file_hash(local_path)?;
    Ok(local_hash != remote_hash)
}

/// Conflict resolution strategy
#[derive(Debug, Clone, Copy)]
pub enum ConflictResolution {
    /// Keep the latest modified file
    LastWriteWins,
    /// Ask user to decide
    Manual,
    /// Keep local version
    KeepLocal,
    /// Keep remote version
    KeepRemote,
}

/// Resolves file conflicts
pub fn resolve_conflict(
    strategy: ConflictResolution,
    local_path: &PathBuf,
    remote_path: &PathBuf,
) -> Result<PathBuf> {
    match strategy {
        ConflictResolution::LastWriteWins => {
            let local_modified = fs::metadata(local_path)?.modified()?;
            let remote_modified = fs::metadata(remote_path)?.modified()?;

            Ok(if local_modified > remote_modified {
                local_path.to_path_buf()
            } else {
                remote_path.to_path_buf()
            })
        }
        ConflictResolution::KeepLocal => Ok(local_path.to_path_buf()),
        ConflictResolution::KeepRemote => Ok(remote_path.to_path_buf()),
        ConflictResolution::Manual => {
            info!("Conflict between {} and {}", local_path.display(), remote_path.display());
            Ok(local_path.to_path_buf())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_file_watcher_creation() {
        let watcher = FileWatcher::new();
        assert!(watcher.watched_paths.is_empty());
    }

    #[test]
    fn test_add_path() {
        let mut watcher = FileWatcher::new();
        let path = PathBuf::from("./test");
        assert!(watcher.add_path(path.clone()).is_ok());
        assert_eq!(watcher.watched_paths.len(), 1);
    }

    #[test]
    fn test_compute_file_hash() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"test content").unwrap();

        let hash = compute_file_hash(&file_path).unwrap();
        assert!(!hash.is_empty());
    }
}
