#![allow(non_camel_case_types)]


use std::sync::LazyLock;
use std::fs::File;
use std::io::BufReader;

use serde_json::from_reader;

use crate::strcts::SETTINGS;


pub static S: LazyLock<SETTINGS> = LazyLock::new(|| {
    let rdr = BufReader::new(File::open("sttngs.json").expect("sttngs not found"));
    from_reader(rdr).expect("sttngs not decerialized")
});
