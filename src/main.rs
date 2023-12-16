mod classloader;
mod entity;
mod jvm;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut jvm = jvm::JVM {
        option: jvm::JVMOption {
            class_pathes: vec![],
            properties: std::collections::HashMap::new(),
        },
        entry_point_path: "Main.class".to_string(),
        class_files: std::sync::Mutex::new(std::collections::HashMap::new()),
    };
    jvm.run();

    let end = start.elapsed();
    println!("Running time: {}.{:03} sec", end.as_secs(), end.subsec_nanos() / 1_000_000);
}