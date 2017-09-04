extern crate walkdir;

mod state;
mod error;
pub mod plugin;
pub mod env;

pub use error::Error;
pub use error::Result;

use std::io::{self, Read};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs::{self, DirBuilder, File};

pub trait ActionType {
    fn id(&self) -> &'static str;
    fn boxed(self) -> Box<ActionType>;
}

pub trait ActionConfig {
    fn type_id(&self) -> &'static str;
    fn boxed(self) -> Box<ActionConfig>;
}

pub struct Bundler {
    action_types: HashMap<&'static str, Box<ActionType>>,
}

impl Bundler {
    pub fn with_action_type<T: ActionType + 'static>(mut self, action: T) -> Self {
        self.action_types.insert(action.id(), action.boxed());
        self
    }

    pub fn add_actions(&mut self, actions: &[Box<ActionConfig>]) -> Result<()> {
        let state = state::BundleState::new(&env::bundler_dir()?)?;
        Ok(())
    }

    pub fn set_target_rel_path(&mut self, rel_path: &[&str]) -> Result<()> {
        let mut state = state::BundleState::new(&env::bundler_dir()?)?;

        let mut output_dir = env::target_dir()?.join(
            ::std::env::var("PROFILE")
                .map_err(|e| Error::Env { message: "failed to find PROFILE env var".into(), err: Some(e) })?
        );

        for path_part in rel_path {
            output_dir = output_dir.join(path_part);
        }

        state.set_target_path(&output_dir)?;

        Ok(())
    }
}

impl Default for Bundler {
    fn default() -> Self {
        Bundler {
            action_types: HashMap::new(),
        }
            .with_action_type(plugin::Copy::new())
    }
}

//use bincode::rustc_serialize::{encode, decode};
//
//use std::env;
//use std::io::{self, Read};
//use std::path::{Path, PathBuf};
//use std::fs::{self, DirBuilder, File};
//
//#[derive(Debug, RustcEncodable, RustcDecodable, Clone, PartialEq)]
//pub enum Action {
//    Copy {
//        from_abs_dir: PathBuf,
//        to_rel_dir: PathBuf
//    },
//}
//
//// TODO: bundle_from and budle_to may be executed in arbitrary order make sure both can perform all actions eagerly
//
///// Register bundler actions.
//pub fn bundle_from(actions: &[Action]) {
//    let change_set = ChangeSet::new(actions.iter().cloned().collect());
//
//    for action in actions {
//        match *action {
//            Action::Copy { ref from_abs_dir, ref to_rel_dir } => println!("cargo:warning=bundle from {:?} to {:?}", from_abs_dir, to_rel_dir),
//        }
//    }
//
//    let out_bundler_dir = get_bundler_dir();
//    if !out_bundler_dir.exists() {
//        DirBuilder::new().create(&out_bundler_dir)
//            .expect(&format!("failed to create bundler output dir {:?}", out_bundler_dir));
//    }
//
//    let out_file = out_bundler_dir.join(
//        format!("{}.changeset", env::var("CARGO_PKG_NAME")
//            .expect("failed to get CARGO_PKG_NAME from env vars"))
//    );
//
//    for action in &change_set.actions {
//        match *action {
//            Action::Copy { ref from_abs_dir, .. } => println!("cargo:rerun-if-changed={}", from_abs_dir.to_string_lossy()),
//        }
//    }
//
//    let encoded: Vec<u8> = encode(&change_set, bincode::SizeLimit::Infinite)
//        .expect("failed to encode change set");
//
//    let mut file = File::create(&out_file)
//        .expect(&format!("failed to create or overwrite file {:?}", &out_file));
//
//    io::copy(&mut &encoded[..], &mut file)
//        .expect(&format!("failed to write changeset bytes to {:?}", out_file));
//}
//
///// Perform bundler actions and output file into `target/PROFILE/<rel_path>` dir.
//pub fn bundle_to(rel_path: &[&str]) {
//    let out_bundler_dir = get_bundler_dir();
//    if !out_bundler_dir.exists() {
//        return;
//    }
//
//    let mut output_dir = get_target_dir().join(
//        env::var("PROFILE")
//            .expect("failed to find PROFILE env var")
//    );
//
//    for path_part in rel_path {
//        output_dir = output_dir.join(path_part);
//    }
//
//    for entry in fs::read_dir(&out_bundler_dir).expect(&format!("failed to read dir {:?}", &out_bundler_dir)) {
//        let entry = entry.expect(&format!("failed to get dir entry for {:?}", &out_bundler_dir));
//        let entry_type = entry.file_type().expect(&format!("failed to get dir entry {:?} type", &entry.path()));
//        if entry_type.is_file() && entry.path().extension().map(|v| v == "changeset") == Some(true) {
//            let mut bytes = Vec::new();
//            File::open(entry.path())
//                .expect(&format!("failed to open file {:?}", entry.path()))
//                .read_to_end(&mut bytes)
//                .expect(&format!("failed to read file {:?}", entry.path()));
//            let change_set: ChangeSet = decode(&bytes)
//                .expect(&format!("failed to decode file {:?}", entry.path()));
//            if !change_set.is_valid() {
//                continue;
//            }
//            println!("cargo:rerun-if-changed={}", entry.path().to_string_lossy());
//            for action in change_set.actions {
//                perform_action(&output_dir, action);
//            }
//        }
//    }
//}
//
///// Create copy action for bundler.
//pub fn copy(from_rel_dir: &[&str], to_rel_dir: &[&str]) -> Action {
//    let mut from_abs_path = PathBuf::from(
//        env::var("CARGO_MANIFEST_DIR")
//            .expect("failed to find CARGO_MANIFEST_DIR env var"));
//
//    for rel_part in from_rel_dir {
//        from_abs_path = from_abs_path.join(rel_part);
//    }
//
//    let mut to_rel_path = PathBuf::new();
//    for rel_part in to_rel_dir {
//        to_rel_path = to_rel_path.join(rel_part);
//    }
//
//    Action::Copy {
//        from_abs_dir: from_abs_path,
//        to_rel_dir: to_rel_path,
//    }
//}
//
//fn perform_action(output_dir: &Path, action: Action) {
//    match action {
//        Action::Copy { from_abs_dir, to_rel_dir } => {
//            let target_abs_dir = output_dir.join(to_rel_dir);
//
//            println!("cargo:rerun-if-changed={}", from_abs_dir.to_string_lossy());
//            println!("cargo:warning=copy from {} to {}", from_abs_dir.to_string_lossy(), target_abs_dir.to_string_lossy());
//
//            for entry in walkdir::WalkDir::new(&from_abs_dir) {
//                let entry = entry.expect("failed to walk dir entry");
//                let path = entry.path();
//                if let Ok(tail) = path.strip_prefix(&from_abs_dir) {
//                    let target_path = target_abs_dir.join(tail);
//                    if entry.file_type().is_dir() {
//                        if !target_path.is_dir() {
//                            match fs::DirBuilder::new().create(&target_path) {
//                                Err(e) => panic!("failed to create dir {:?}, {:?}", &target_path, e),
//                                _ => (),
//                            };
//                        }
//                    } else {
//                        match fs::copy(&path, &target_path) {
//                            Err(e) => panic!("failed to copy file from dir {:?} to {:?}, {:?}", &path, &target_path, e),
//                            _ => (),
//                        };
//                    }
//                }
//            }
//        }
//    }
//}
//
//#[derive(Debug, RustcEncodable, RustcDecodable, PartialEq)]
//struct ChangeSet {
//    version: u32,
//    actions: Vec<Action>,
//}
//
//impl ChangeSet {
//    fn new(actions: Vec<Action>) -> ChangeSet {
//        ChangeSet {
//            version: 1,
//            actions: actions,
//        }
//    }
//
//    fn is_valid(&self) -> bool {
//        self.version == 1
//    }
//}