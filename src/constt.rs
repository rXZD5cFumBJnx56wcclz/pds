#![allow(non_camel_case_types)]

use std::fs::File;
use std::io::BufReader;
use std::sync::LazyLock;

use serde_json5::from_reader;

use crate::strcts::SETTINGS;

pub static S: LazyLock<SETTINGS> = LazyLock::new(|| {
    let mut rdr = BufReader::new(File::open("sttngs.json").expect("sttngs not found"));
    from_reader(&mut rdr).expect("sttngs not decerialized")
});
