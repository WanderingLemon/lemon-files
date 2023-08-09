mod util;

use std::fs;
use std::os::unix::prelude::{PermissionsExt};

fn main() {

    let dir = fs::read_dir(".").unwrap();
    for entry in dir {
        let entry = entry.unwrap();
        let meta = entry.metadata().unwrap();
        let permissions = meta.permissions().mode();

        println!("{}", String::from(entry.path().to_string_lossy().strip_prefix("./").unwrap()));
    }
}
