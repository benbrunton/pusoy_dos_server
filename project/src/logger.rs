use std::fmt::Display;

pub fn info<S: Display>(msg: S){
    println!("[INFO] {}", msg);
}

pub fn warn<S: Display>(msg: S){
    println!("[WARN] {}", msg);
}

pub fn error<S: Display>(msg: S){
    println!("[ERR] {}", msg );
}
