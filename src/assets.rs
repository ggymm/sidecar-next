use std::{borrow::Cow, path::PathBuf};

use anyhow::Result;
use gpui::AssetSource;

/// Simple filesystem-backed assets loader reading from the `assets/` folder.
pub struct FsAssets;

impl AssetSource for FsAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let full = manifest_dir.join("assets").join(path);

        match std::fs::read(full) {
            Ok(data) => Ok(Some(Cow::Owned(data))),
            Err(_) => Ok(None),
        }
    }

    fn list(&self, _path: &str) -> gpui::Result<Vec<gpui::SharedString>> {
        Ok(Vec::new())
    }
}

