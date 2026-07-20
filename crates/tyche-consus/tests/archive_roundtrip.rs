//! Consus adapter contracts.

use consus_zarr::InMemoryStore;
use tyche_consus::{ArtifactKey, ArtifactRead, ArtifactWrite, ConsusArchive};

#[test]
fn exact_byte_roundtrip_and_key_safety() {
    let mut store = InMemoryStore::new();
    let key = ArtifactKey::borrowed("studies/reference/responses/00000003").expect("valid");
    let payload = [0_u8, 1, 2, 127, 128, 255];
    let mut archive = ConsusArchive::new(&mut store);
    archive.write(&key, &payload).expect("write");
    assert_eq!(archive.read(&key).expect("read").as_ref(), payload);
    assert!(key.is_borrowed());
    for invalid in [
        "",
        "/absolute",
        r"C:\absolute",
        "study/../secret",
        "study//response",
        "./study",
        "scheme:key",
    ] {
        assert!(ArtifactKey::borrowed(invalid).is_err(), "{invalid:?}");
    }
}
