pub mod hash {
    use std::io::Write;
    use crypto_hash::{Hasher, Algorithm};

    pub fn new() -> Hasher {
        Hasher::new(Algorithm::SHA256)
    }

    pub fn write(hasher: &mut Hasher, val: &[u8]) {
        hasher.write_all(val).expect("failed to write bytes to hasher");
    }

    pub fn write_path(hasher: &mut Hasher, val: &::std::path::Path) {
        write(hasher,val.to_string_lossy().into_owned().as_bytes())
    }

    pub fn write_str(hasher: &mut Hasher, val: &str) {
        hasher.write_all(val.as_bytes()).expect("failed to write slice to hasher");
    }

    pub fn write_slice_of_str(hasher: &mut Hasher, items: &[&str]) {
        for item in items {
            write_str(hasher, item);
        }
    }
}