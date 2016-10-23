use std::fmt::Display;

pub fn info<S: Display>(msg: S){
    warn!("logger deprecated");
    info!("[INFO] {}", msg);
}

pub fn warn<S: Display>(msg: S){
    warn!("logger deprecated");
    warn!("[WARN] {}", msg);
}

#[allow(dead_code)]
pub fn error<S: Display>(msg: S){
    warn!("logger deprecated");
    error!("[ERR] {}", msg );
}
