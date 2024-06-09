use std::{env, path::Path};

use transform_ci::transform_path;

fn main() {
    let path = env::args().nth(1).unwrap();
    let path = Path::new(&path);
    let _ = transform_path(path);
}
