use std::fs;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Iterator;

pub fn first_line(lines: Vec<String>) -> String {
    return match lines.first() {
        Some(x) => x.to_string(),
        None => "".to_string(),
    };
}

pub fn file_to_vec(filename: String) -> io::Result<Vec<String>> {
    let file_in = fs::File::open(filename)?;
    let file_reader = BufReader::new(file_in);
    Ok(file_reader.lines().filter_map(io::Result::ok).collect())
}

pub fn comma_separated_ints_to_vec(line: String) -> Vec<i32> {
    line.split(",")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

pub fn vec_to_comma_separated_ints(vec: Vec<i32>) -> String {
    let list: Vec<_> = vec.into_iter().map(|i| i.to_string()).collect();
    return list.join(",");
}
