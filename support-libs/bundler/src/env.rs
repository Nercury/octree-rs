use {Result, Error};
use std::env;
use std::path::{PathBuf};

pub fn bundler_dir() -> Result<PathBuf> {
    Ok(target_dir()?.join("bundler"))
}

pub fn target_dir() -> Result<PathBuf> {
    let out_dir = PathBuf::from(
        env::var("OUT_DIR")
            .map_err(|e| Error::Env { message: "failed to find OUT_DIR env var".into(), err: Some(e) })?
    );

    let mut target_dir = out_dir;

    loop {
        if target_dir.ends_with("target") {
            break;
        }

        target_dir = match target_dir.parent() {
            Some(parent) => parent.into(),
            None => return Err(Error::Env { message: "failed to find target build dir".into(), err: None }),
        };
    }

    Ok(target_dir)
}