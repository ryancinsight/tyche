//! Provider-neutral artifact access ports.

use crate::ArtifactKey;

/// Read access to study artifacts.
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
    /// Returns the provider failure.
    fn read<'a>(&'a self, key: &ArtifactKey<'_>) -> Result<Self::Bytes<'a>, Self::Error>;
}

/// Write access to study artifacts.
pub trait ArtifactWrite: ArtifactRead {
    /// Write without an intermediate Tyche-owned copy.
    ///
    /// # Errors
    ///
    /// Returns the provider failure.
    fn write(&mut self, key: &ArtifactKey<'_>, bytes: &[u8]) -> Result<(), Self::Error>;
}
