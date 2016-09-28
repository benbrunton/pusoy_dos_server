use iron::prelude::*;
use iron::status;
use url::Url;
use std::io::prelude::*;
use std::fs::File;
use std::string::String;
use hyper::client::Client;
use toml;

pub fn callback(req: &mut Request) -> IronResult<Response> {

    let fb_secret = get_fb_secret();

    let client = Client::new();
    let client_id = "637941239713409";
    let redirect = "http://localhost/auth";


    let url_str = req.url.to_string();
    let url = Url::parse(&url_str).unwrap();

    let mut pairs = url.query_pairs();
    let qs = pairs.find(|&(ref key, _)| { 
        key == "code"
    });
    

    let code = match qs {
        Some((_,code_cow)) => {
            code_cow.into_owned()
        },
        None => "invalid_code".to_string()
    };


    let fb_token_url = format!("https://graph.facebook.com/v2.7/oauth/access_token?client_id={}&redirect_uri={}&client_secret={}&code={}", client_id, redirect, fb_secret, code);

    println!("{:?}", fb_token_url);
    println!("{}", fb_token_url);

    // todo - handle the network error
    let res = client.get(&fb_token_url).send().unwrap();

    println!("{:?}", res);


    Ok(Response::with((status::Ok, "ok")))
}

fn get_fb_secret() -> String{

    let mut f = File::open("config/app_config.toml").unwrap();
    let mut s = String::new();
    let _ = f.read_to_string(&mut s);

    let mut parser = toml::Parser::new(&s);
    let toml = parser.parse().unwrap();
    let fb_secret = toml.get("fb_secret").unwrap().clone().to_string();

    println!("{:?}", fb_secret);
    println!("{}", fb_secret);

    fb_secret
}

