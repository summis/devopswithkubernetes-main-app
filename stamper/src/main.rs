use std::fs::File;
use std::{thread, time};
use std::io::{Write};
use chrono::prelude::*;


fn write_file(path: &str, content: &str) {
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path, why),
        Ok(_) => (),
    }
}


fn main() {
    loop {
        let path = "/var/log/main-app/timestamp.txt";
        let now = &Utc::now().to_rfc2822();
        write_file(path, now);
        thread::sleep(time::Duration::from_secs(5));
    }
}
