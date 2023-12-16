use std::{collections::HashMap, sync::Mutex};
use std::fs;
use std::fs::File;
use std::io::Read;

use crate::entity::java_class_file::JavaClassFileFormat;
use crate::classloader::bootstrap_class_loader::define_class;
use byteorder::{BigEndian, ReadBytesExt};

pub struct JVMOption {
    pub class_pathes: Vec<String>,
    pub properties: HashMap<String, String>,
}

pub struct JVM {
    pub option: JVMOption,
    pub entry_point_path: String,
    pub class_files: Mutex<HashMap<String, JavaClassFileFormat>>,
}

impl JVM {
    
    pub fn run(&mut self) {
        let mut buffer = &self.read_file(&self.entry_point_path)[..];
        let buffer2 = buffer.clone();
        let magic = buffer.read_u32::<BigEndian>().unwrap();
        
        // is class file
        if magic == 0xCAFEBABE {
            define_class(&self.entry_point_path, buffer2, self.class_files.get_mut().unwrap());
            //dbg!(&self.class_files);
        
        // is jar file
        } else if magic == 0x504B0304 || magic == 0x504B0506 || magic == 0x504B0708 {
            // TODO
        }
    }

    fn read_file(&self, filename: &String) -> Vec<u8> {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }


}