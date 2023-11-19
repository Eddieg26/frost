use std::{ffi::OsStr, path::Path};

pub trait PathExt {
    fn trim_prefix(&self, prefix: &str) -> String;
    fn visit_dirs<T>(&self, cb: &mut dyn FnMut(&Path) -> T) -> std::io::Result<()>;
    fn extension_str(&self) -> &str;
    fn standardize(&self) -> String;
}

impl PathExt for Path {
    fn trim_prefix(&self, prefix: &str) -> String {
        self.strip_prefix(prefix)
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
            .replace("\\", "/")
    }

    fn standardize(&self) -> String {
        self.to_str().unwrap().to_owned().replace("\\", "/")
    }

    fn visit_dirs<T>(&self, cb: &mut dyn FnMut(&Path) -> T) -> std::io::Result<()> {
        if self.is_dir() {
            for entry in std::fs::read_dir(self)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.visit_dirs(cb)?;
                } else {
                    cb(entry.path().as_path());
                }
            }
        } else if self.is_file() {
            cb(self);
        }

        Ok(())
    }

    fn extension_str(&self) -> &str {
        self.extension()
            .unwrap_or(OsStr::new(""))
            .to_str()
            .unwrap_or("")
    }
}
