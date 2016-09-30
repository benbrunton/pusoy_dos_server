use url::Url;

pub fn get(url_str: String, key: &'static str) -> Option<String>{
        let url = Url::parse(&url_str).unwrap();

        let mut pairs = url.query_pairs();
        let qs = pairs.find(|&(ref k, _)| { 
            k == key
        });
        

        match qs {
            Some((_,code_cow)) => {
                Some(code_cow.into_owned())
            },
            None => None
        }
}

