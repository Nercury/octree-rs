use std::path::{PathBuf, Path};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::ffi::{CString};
use std::fs::File;
use std::env;
use std::io::Read;

#[derive(Clone)]
pub struct Resources {
    inner: Arc<Mutex<InnerResources>>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            inner: Arc::new(Mutex::new(InnerResources::new())),
        }
    }

    pub fn with_rel_mount(self, s: &str) -> Resources {
        let executable_path = env::current_exe()
            .expect("failed to get current executable path");
        let mut dir = PathBuf::from(executable_path.parent()
            .expect("failed to get current executable parent dir"));

        for part in s.split("/") {
            if !part.is_empty() {
                dir = dir.join(part);
            }
        }

        self.inner.lock().unwrap()
            .mounts.push_front(
            dir.canonicalize().expect("invalid path")
        );

        self
    }

    /// Get resource contents as CString.
    pub fn get_cstring(&self, location: &str) -> CString {
        self.inner.lock().unwrap()
            .get_cstring(location)
    }

    /// Get resource contents as CString.
    pub fn get_file_path(&self, location: &str) -> PathBuf {
        self.inner.lock().unwrap()
            .get_file_path(location)
    }
}

struct InnerResources {
    mounts: VecDeque<PathBuf>,
}

impl InnerResources {
    pub fn new() -> InnerResources {
        InnerResources {
            mounts: VecDeque::new(),
        }
    }

    pub fn get_file_path(&self, location: &str) -> PathBuf {
        for mount in &self.mounts {
            let path = get_location_path(&mount, location);
            if path.exists() {
                return path;
            }
        }

        let look_at_list: Vec<_> = self.mounts.iter().map(|m| get_location_path(m, location)).collect();

        panic!(
            "failed to find file {:?}, looked at:\n    {}\n",
            location,
            look_at_list
                .iter()
                .map(|p| format!("{}", p.to_str().expect("file name contains null byte")))
                .collect::<Vec<String>>()
                .join("\n")
        );
    }

    fn get_mount_file(&self, location: &str) -> File {
        let path = self.get_file_path(location);
        match File::open(&path) {
            Ok(f) => return f,
            Err(e) => panic!("failed to open file {:?}, {:?}", path, e),
        }
    }

    pub fn get_cstring(&self, location: &str) -> CString {
        let mut file = self.get_mount_file(location);

        let mut data = Vec::new();
        if let Err(e) = file.read_to_end(&mut data) {
            panic!("failed to read file {:?}, {:?}", location, e);
        }

        match CString::new(data) {
            Ok(s) => s,
            Err(e) => panic!("invalid CString in file {:?}, {:?}", location, e),
        }
    }
}

fn get_location_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}