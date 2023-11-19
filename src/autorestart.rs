use chksum::{chksum, hash::MD5};
use std::{
    collections::hash_map::DefaultHasher,
    hash::Hasher,
    os::unix::process::CommandExt,
    path::Path,
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

static HASH: AtomicU64 = AtomicU64::new(u64::MAX);

pub fn update() {
    let digest = chksum::<MD5, _>(Path::new(env!("CARGO_MANIFEST_DIR")).join("src"))
        .expect("Failed to create digest");
    let bytes: [u8; 16] = digest.into();
    let hash = {
        let mut hasher = DefaultHasher::new();
        hasher.write_u128(u128::from_ne_bytes(bytes));
        hasher.finish()
    };

    let stored_hash = HASH.load(Ordering::Relaxed);
    if stored_hash != u64::MAX && hash != stored_hash {
        println!("\x1B[1;38;5;210mVeränderung erkannt\x1B[0m");
        println!("\x1B[1;38;5;154mServer wird neugestartet... \x1B[0m");
        Command::new("cargo").arg("run").exec();
    }

    HASH.store(hash, Ordering::Relaxed)
}