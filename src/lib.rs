//! Use ADB commands without touching the command line, all within your favourite
//! programming language.
//!
//! ## Usage sample
//! Get all the photos taken on a certain day
//! ```rust
//! use adb_utils::{manager::ADBManager, shell::ADBList};
//!
//! let mut manager = ADBManager::new();
//! manager.cwd("/storage/emulated/0/DCIM/Camera");
//! if let Err(err) = manager.connect("192.168.1.133", 36415) {
//!     println!("Could not connect: {err}");
//!     return;
//! }
//! let mut list = ADBList::default();
//! let files = manager.execute_path_based(&mut list).unwrap().to_vec();
//! files
//!     .iter()
//!     .filter(|file| file.starts_with("20230827"))
//!     .for_each(|file| println!("{file}"));
//! // 20230827_132733.jpg
//! // 20230827_141248.jpg
//! ```

mod commands;
pub mod manager;

pub use commands::*;
