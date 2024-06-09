use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use ignore::Walk;
use oxc::span::SourceType;
use transform_ci::transform;

pub struct Runner {
    dirs: Vec<PathBuf>,
    count: usize,
}

impl Runner {
    pub fn new(dirs: Vec<PathBuf>) -> Self {
        Self { dirs, count: 0 }
    }

    pub fn run(mut self) -> ExitCode {
        for dir in self.dirs.clone() {
            println!("Processing {:?}", dir);
            self.walk(&dir);
        }

        if self.count > 0 {
            println!("Transformed {:?} files", self.count);
            ExitCode::SUCCESS
        } else {
            eprintln!("No files were transformed");
            ExitCode::FAILURE
        }
    }

    fn walk(&mut self, dir: &Path) {
        for entry in Walk::new(dir) {
            let dir_entry = entry.unwrap();
            let path = dir_entry.path();
            if !path.is_file() {
                continue;
            }
            let Ok(source_type) = SourceType::from_path(path) else {
                continue;
            };
            if source_type.is_typescript_definition() {
                continue;
            }
            let source_text = fs::read_to_string(path).unwrap();
            let source_text2 = transform(path, &source_text, source_type);

            let new_extension = path.extension().unwrap().to_str().unwrap().replace('t', "j");
            let new_path = path.with_extension(new_extension);
            let source_type2 = SourceType::default();

            // idempotency test
            let source_text3 = transform(path, &source_text2, source_type2);
            assert_eq!(source_text2, source_text3, "Idempotency test failed: {path:?}");

            fs::write(&new_path, source_text3).unwrap();
            fs::remove_file(path).unwrap();
            self.count += 1;
        }
    }
}

fn main() -> ExitCode {
    let dirs = env::args().skip(1).map(PathBuf::from).collect::<Vec<PathBuf>>();
    assert!(!dirs.is_empty(), "Expected directories");
    Runner::new(dirs).run()
}
