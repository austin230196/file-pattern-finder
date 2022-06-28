use std::{env, fs, path::Path};


#[derive(Debug)]
pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool
}


impl Config {
    pub fn new(pattern: String, filename: String)-> Self {
        let is_case_insensitive = env::var("CASE_INSENSITIVE").is_ok();
        Self {
            pattern,
            filename,
            case_sensitive: !is_case_insensitive
        }
    }
}



//Implementing this maually kept causing panicfatal runtime error: stack overflow
// zsh: abort      cargo run humpty poem.pdf
// impl fmt::Display for Config {
//     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//         write!(f, "{}", self)
//     }
// }


// impl fmt::Debug for Config {
//     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//         write!(f, "{:}", self)
//     }
// }



pub mod helpers {
    use super::*;
    pub fn get_config<'a>() -> Result<(String, String), &'a str> {
        //first we get the command line arguments
        let args = env::args().collect::<Vec<String>>();

        //we check if the  command line arguments are more than or equal to 3
        if args.len() < 3 {
            return Err("Invalid number of arguments passed");
        }else {
            //at this point the arguments passed by the user is greater or equals to 3
            //then we get the query and filename
            let query = &args[1];
            let filename = &args[2];

            Ok((query.to_owned(), filename.to_owned()))
        }
    }


    pub fn get_file_contents(filename: String) -> Result<String, std::io::Error> {
        let path = Path::new("./").join(filename);
        let file_content = fs::read_to_string(path);
        file_content
    }


    pub fn search<'a, 'b>(content: &'a str, pattern: &'b str) -> Result<Vec<String>, &'a str> {
        let mut res = vec![];
        for l in content.lines() {
            if l.contains(pattern) {
                res.push(l.to_string());
            }else{continue;}
        };
        if res.len() == 0 {
            return Err("No pattern found");
        }else {
            Ok(res)
        }
    }

    pub fn search_insensitive<'a, 'b>(content: &'a str, pattern: &'b str) -> Result<Vec<String>, &'a str> {
        let mut res = Vec::new();
        for l in content.lines() {
            if l.to_lowercase().contains(&pattern.to_lowercase()) {
                res.push(l.to_string());
            }else {continue;}
        };

        if res.len() == 0 {
            return Err("No pattern found");
        }else {
            return Ok(res)
        }
    }
}