//! Consus Zarr store adapter.

use crate::{ArtifactKey, ArtifactRead, ArtifactWrite};
use consus_zarr::Store;

/// Pointer-sized borrowed Consus store adapter.
#[must_use]
#[repr(transparent)]
pub struct ConsusArchive<'store, S> {
    store: &'store mut S,
}

impl<'store, S: Store> ConsusArchive<'store, S> {
    /// Borrow the store.
    pub const fn new(store: &'store mut S) -> Self {
        Self { store }
    }
}

impl<S: Store> ArtifactRead for ConsusArchive<'_, S> {
    type Error = consus_core::Error;
    type Bytes<'a>
        = Vec<u8>
    where
        Self: 'a;
    fn read<'a>(&'a self, key: &ArtifactKey<'_>) -> Result<Self::Bytes<'a>, Self::Error> {
        self.store.get(key.as_str())
    }
}

impl<S: Store> ArtifactWrite for ConsusArchive<'_, S> {
    fn write(&mut self, key: &ArtifactKey<'_>, bytes: &[u8]) -> Result<(), Self::Error> {
        self.store.set(key.as_str(), bytes)
    }
}
