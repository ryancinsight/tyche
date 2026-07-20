//! Consus adapter and key-safety integration.

use consus_zarr::InMemoryStore;
use tyche_consus::{ArtifactKey, ArtifactRead, ArtifactWrite, ConsusArchive};

#[test]
fn in_memory_consus_roundtrip_preserves_exact_bytes() {
    let mut store = InMemoryStore::new();
    let key = ArtifactKey::borrowed("studies/reference/responses/00000003").expect("canonical key");
    let payload = [0_u8, 1, 2, 127, 128, 255];

    {
        let mut archive = ConsusArchive::new(&mut store);
        archive.write(&key, &payload).expect("store write");
        let replay = archive.read(&key).expect("store read");
        assert_eq!(replay.as_ref(), payload);
    }
}

#[test]
fn traversal_and_platform_paths_are_rejected() {
    for invalid in [
        "",
        "/absolute",
        r"C:\absolute",
        "study/../secret",
        "study//response",
        "./study",
        "scheme:key",
    ] {
        assert!(
            ArtifactKey::borrowed(invalid).is_err(),
            "{invalid:?} must be rejected"
        );
    }
}

#[test]
fn borrowed_key_preserves_pointer_identity() {
    let source = "studies/reference/manifest";
    let key = ArtifactKey::borrowed(source).expect("canonical key");
    assert!(key.is_borrowed());
    assert!(core::ptr::eq(key.as_str().as_ptr(), source.as_ptr()));
}
