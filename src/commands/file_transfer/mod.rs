pub mod pull;
pub mod push;

pub use pull::*;
pub use push::*;

use std::fmt::Display;

/// Compression algorithm that can be used by all file transfer commands
pub enum CompressionAlgorithm {
    Any,
    None,
    Brotli,
    Lz4,
    Zstd,
}

impl Display for CompressionAlgorithm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CompressionAlgorithm::Any => "any",
                CompressionAlgorithm::None => "none",
                CompressionAlgorithm::Brotli => "brotli",
                CompressionAlgorithm::Lz4 => "lz4",
                CompressionAlgorithm::Zstd => "zstd",
            }
        )
    }
}
