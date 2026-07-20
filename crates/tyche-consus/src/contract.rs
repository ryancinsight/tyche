//! Provider-neutral artifact access ports.

use crate::ArtifactKey;

/// Read access to versioned study artifacts.
pub trait ArtifactRead {
    /// Provider failure.
    type Error;
    /// Borrowed or owned byte family.
    type Bytes<'a>: AsRef<[u8]>
    where
        Self: 'a;

    /// Read one validated key.
    ///
    /// # Errors
    ///
    /// Returns the provider's typed failure.
    fn read<'a>(&'a self, key: &ArtifactKey<'_>) -> Result<Self::Bytes<'a>, Self::Error>;
}

/// Write access to versioned study artifacts.
pub trait ArtifactWrite: ArtifactRead {
    /// Write one byte slice without an intermediate Tyche-owned copy.
    ///
    /// # Errors
    ///
    /// Returns the provider's typed failure.
    fn write(&mut self, key: &ArtifactKey<'_>, bytes: &[u8]) -> Result<(), Self::Error>;
}
