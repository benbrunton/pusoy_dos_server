
use std::io::prelude::*;
use std::fs::File;
use std::collections::BTreeMap;
use std::string::String;
use std::env;

use toml;

pub struct Config{
    store: BTreeMap<String, toml::Value>
}

impl Config {

    pub fn new() -> Config{

        let mut f = File::open("config/app_config.toml").unwrap();
        let mut s = String::new();
        let _ = f.read_to_string(&mut s);

        let mut parser = toml::Parser::new(&s);
        let toml = parser.parse().unwrap();

        Config {
            store : toml
        }

    }

    pub fn get(&self, key: &'static str) -> Option<String> {

        let uppercase_key = key.to_string().to_uppercase();
        match env::var(&uppercase_key) {
            Ok(lang) => return Some(lang.to_string()),
            Err(e) => ()
        }; 

        match self.store.get(key){
            Some(val) => {
                let new_str = val.to_string();
                let trimmed_str = new_str.trim_matches('"').to_string();
                Some(trimmed_str)
            },
            None => None
        }

    }
}

#[test]
pub fn creating_a_new_config_loads_the_file(){
    Config::new();
}

#[test]
pub fn get_retrieves_values_by_key(){
    let config = Config::new();

    let name = config.get("name");
    let author = config.get("author");

    assert_eq!(author.unwrap(), "Ben Brunton");
    assert_eq!(name.unwrap(), "Pusoy Dos");
}
