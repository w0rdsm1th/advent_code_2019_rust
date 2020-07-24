/*
shared function to read a days input
*/

use std::fs;
use std::path::Path;

// CSV reader implementation
// pub fn input_reader(fname: &str) -> Vec<i32> {
//     // read arbitrary input file, return single Vector
//     let mut rdr = csv::Reader::from_reader(fname);
//     let mut out_vec = Vec::new();
//     for result in rdr.records() {
//         let record = result?;
//         out_vec.push(record)
//     }
//     out_vec
// }


fn convert_string_signed_int(inp: &str) -> i32 {
    // https://stackoverflow.com/a/43985962/3596968
    // let radix = 10;
    // let temp_1 = inp.chars().map(|c| c.to_digit(radix).unwrap());
    // inp.chars().map(|c| c.to_digit(radix).unwrap()).sum::<u32>()
    inp.parse().expect("failed to parse")
}

pub fn input_reader_raw_string(fname: &str) -> String {
    // read arbitrary input file, return single string
    let fpath = Path::new("inputs").join(fname);
    fs::read_to_string(fpath).expect("Something went wrong reading the file")
}

pub fn input_reader_lines_convert_int(fname: &str) -> Vec<i32> {
    let contents = input_reader_raw_string(fname);
    // split on lines, convert to unsigned integers
    // https://stackoverflow.com/a/38138985/3596968
    contents.lines().map(|num| convert_string_signed_int(num)).collect()
}

pub fn input_reader_commas(fname: &str) -> Vec<i32> {
    let contents = input_reader_raw_string(fname);
    // split on lines, convert to unsigned integers
    // https://stackoverflow.com/a/38138985/3596968
    contents.split(",").map(|num| convert_string_signed_int(num)).collect()
}