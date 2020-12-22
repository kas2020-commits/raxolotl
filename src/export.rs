use std::fs::File;
use std::io::prelude::*;

/// exports data into digestable format for matlab
pub fn vec_export_matlab
(data: &Vec<f64>, variable: &str, filename: &str) {
    let mut file = File::create(filename)
        .expect("Could not create file!");
    file.write(variable.as_bytes()).expect("error during write");
    file.write(b" = [\n").expect("error during write");
    for i in data.iter() {
        file.write((i.to_string() + ",\n").as_bytes())
            .expect("error during write");
    }
    file.write(b"]\n").expect("error during write");
}

/// exports data into digestable format for matlab
pub fn mat_export_matlab
(data: &Vec<Vec<f64>>, variable: &str, filename: &str) {
    let mut file = File::create(filename)
        .expect("Could not create file!");
    file.write(variable.as_bytes()).expect("error during write");
    file.write(b" = [\n").expect("error during write");
    for i in data.iter() {
        for k in i.iter() {
            file.write((k.to_string() + ",").as_bytes())
                .expect("error during write");
        }
        file.write(b"\n").expect("error during write");
    }
    file.write(b"]\n").expect("error during write");
}
