//! Advisory file locking for concurrent write protection
//!
//! Uses fs2 for cross-platform file locking to prevent race conditions
//! when multiple CLI processes modify the graph simultaneously.

use anyhow::{anyhow, Result};
use colored::Colorize;
use fs2::FileExt;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

const LOCK_FILENAME: &str = ".dg.lock";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const RETRY_INTERVAL_MS: u64 = 100;

/// Advisory lock for graph-modifying operations
pub struct GraphLock {
    file: File,
    path: PathBuf,
}

impl GraphLock {
    /// Acquire an exclusive lock on the docs directory.
    ///
    /// If `force` is true, skip locking entirely.
    /// If the docs directory doesn't exist yet, skip locking (init will create it).
    /// Otherwise, try to acquire lock with timeout.
    pub fn acquire(docs_dir: &Path, force: bool) -> Result<Option<Self>> {
        if force {
            return Ok(None);
        }

        // Skip locking if docs directory doesn't exist yet (e.g., init command)
        if !docs_dir.exists() {
            return Ok(None);
        }

        let lock_path = docs_dir.join(LOCK_FILENAME);

        // Create/open the lock file
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .open(&lock_path)
            .map_err(|e| anyhow!("Failed to open lock file: {}", e))?;

        // Try non-blocking lock first
        match file.try_lock_exclusive() {
            Ok(()) => {
                return Ok(Some(GraphLock {
                    file,
                    path: lock_path,
                }));
            }
            Err(_) => {
                // Lock is held, enter wait loop
            }
        }

        // Wait with timeout
        let timeout = Duration::from_secs(DEFAULT_TIMEOUT_SECS);
        let retry_interval = Duration::from_millis(RETRY_INTERVAL_MS);
        let start = Instant::now();

        eprintln!(
            "{} Waiting for lock (another dg process is running)...",
            "LOCK".yellow().bold()
        );

        loop {
            if start.elapsed() > timeout {
                return Err(anyhow!(
                    "Timeout waiting for lock after {}s.\n\
                     Another dg process may be running or crashed.\n\
                     Use --force to bypass the lock (for recovery only).",
                    timeout.as_secs()
                ));
            }

            match file.try_lock_exclusive() {
                Ok(()) => {
                    eprintln!("{} Lock acquired", "OK".green());
                    return Ok(Some(GraphLock {
                        file,
                        path: lock_path,
                    }));
                }
                Err(_) => {
                    std::thread::sleep(retry_interval);
                }
            }
        }
    }

    /// Release the lock (also happens automatically on drop)
    pub fn release(self) -> Result<()> {
        self.file.unlock()?;
        Ok(())
    }

    /// Get the path to the lock file
    #[allow(dead_code)]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for GraphLock {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_acquire_lock() {
        let dir = tempdir().unwrap();
        let lock = GraphLock::acquire(dir.path(), false).unwrap();
        assert!(lock.is_some());
    }

    #[test]
    fn test_force_bypasses_lock() {
        let dir = tempdir().unwrap();
        let lock = GraphLock::acquire(dir.path(), true).unwrap();
        assert!(lock.is_none());
    }

    #[test]
    fn test_lock_file_created() {
        let dir = tempdir().unwrap();
        let _lock = GraphLock::acquire(dir.path(), false).unwrap();
        assert!(dir.path().join(LOCK_FILENAME).exists());
    }

    #[test]
    fn test_lock_released_on_drop() {
        let dir = tempdir().unwrap();
        {
            let _lock = GraphLock::acquire(dir.path(), false).unwrap();
        }
        // Should be able to acquire again after drop
        let lock2 = GraphLock::acquire(dir.path(), false).unwrap();
        assert!(lock2.is_some());
    }
}
