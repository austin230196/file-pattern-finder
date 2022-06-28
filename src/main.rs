mod lib;

use lib::{Config, helpers};



fn main(){
    let params = helpers::get_config();
    let params = match params {
        Ok(p) => p,
        Err(e) => panic!("{}", e)
    };
    let (pattern, filename) = params;
    let config = Config::new(pattern, filename);
    let file_content = helpers::get_file_contents(config.filename);
    let file_content = match file_content {
        Ok(f) => f,
        Err(e) => panic!("{}", e)
    };
    let res : Vec<String>;
    if config.case_sensitive {
        res = helpers::search(file_content.as_str(), config.pattern.as_str()).unwrap();
    }else {
        res = helpers::search_insensitive(file_content.as_str(), config.pattern.as_str()).unwrap();
    }

    println!("{:#?}", res);
}