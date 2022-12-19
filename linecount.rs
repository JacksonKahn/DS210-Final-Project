extern crate csv;
use csv::Reader;
extern crate std;
use std::fs::File;

pub fn csv_length(csv_name : &str, header : bool) -> i32{
    let mut features_reader: Reader<File> = csv::Reader::from_path(csv_name).expect("Unable to open file");
    let n = features_reader.records().count() as i32;
    if !header {return n} else {return n-1};
}