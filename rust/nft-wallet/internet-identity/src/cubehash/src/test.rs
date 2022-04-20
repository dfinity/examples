use super::*;

fn hash(data: &[u8]) -> String {
    let mut h = CubeHash::new();
    h.update(data);
    hex::encode(&h.finalize())
}

// Note:
// The test vectors are mostly coming from https://en.wikipedia.org/wiki/CubeHash,
// but they were also verified by running the reference C implementation.

#[test]
fn test_empty() {
    assert_eq!(
        hash(b""),
        "44c6de3ac6c73c391bf0906cb7482600ec06b216c7c54a2a8688a6a42676577d".to_string()
    );
}

#[test]
fn test_hello() {
    assert_eq!(
        hash(b"hello"),
        "fb638723f74a25864c5ffb1c3480a1e72178bd55337a4248340776aa46f46f10".to_string()
    );
    assert_eq!(
        hash(b"Hello"),
        "e712139e3b892f2f5fe52d0f30d78a0cb16b51b217da0e4acb103dd0856f2db0".to_string()
    );
}

#[test]
fn test_quick_brown_fox() {
    assert_eq!(
        hash(b"The quick brown fox jumps over the lazy dog"),
        "5151e251e348cbbfee46538651c06b138b10eeb71cf6ea6054d7ca5fec82eb79".to_string()
    );
}
