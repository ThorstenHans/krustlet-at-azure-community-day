use std::io::Write;
use std::path::Path;
use std::{thread, time};
use std::fs::File;
use chrono::prelude::*;

fn main() {
    let folder = std::env::var("TARGET").unwrap();
    println!("using folder {}", folder);
    let mut idx = 1;
    while idx < i32::MAX {
        write_a_file(folder.as_str(), idx);

        let d = time::Duration::from_secs(2);
        thread::sleep(d);

        idx +=1;
    }
}

fn write_a_file(folder: &str, iteration: i32) {
    let p = Path::new(folder)
    .join(format!("{}.txt", iteration));

    let mut f = File::create(p).unwrap();
    let now: DateTime<Local> = Local::now();
    writeln!(&mut f, "Hello WASI, according to the 🦀, it is {:?}!", now.to_string()).unwrap();
    
}
