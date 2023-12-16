mod classloader;
mod entity;
use classloader::bootstrap_class_loader;
use std::fs;
use std::fs::File;
use std::io::Read;

fn main() {
    let data = get_file_as_byte_vec(&"Main.class".to_owned());
    bootstrap_class_loader::define_class(&data[..]);
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}
