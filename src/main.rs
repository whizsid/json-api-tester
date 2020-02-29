extern crate http;
extern crate glob;
extern crate url;
extern crate serde_json;

mod variables;
mod config;
mod interpreter;
mod test_case;

use config::Config;
use glob::glob;
use std::fs::read_to_string;
use interpreter::Interpreter;
use serde_json::from_reader;
use serde_json::Value;
use json_comments::StripComments;
use test_case::TestCase;
use url::Url;

fn main() {
    let config = Config::from_file();

    let mut interpreter = Interpreter::new(config.pre_variables);

    let base_url = match config.base_url {
        Some(base)=>{
            base
        }
        None=>{
            String::from("")
        }
    };

    for path in glob(&config.files).unwrap() {
        let path = path.unwrap();
        let name = String::from(path.to_str().unwrap());
        let test_case_str = read_to_string(path).unwrap();
        let test_case_reader = StripComments::new(test_case_str.as_bytes());
        let test_case_json:Value = from_reader(test_case_reader).unwrap();
        let test_case = TestCase::new(name.clone(),test_case_json.as_object().unwrap().to_owned());

        let req_url = match Url::parse(&base_url) {
            Ok(url)=>{
                url.join(&test_case.url()).unwrap()
            }
            Err(_)=>{
                Url::parse(&test_case.url()).unwrap()
            }
        };

        let body = Value::Object(interpreter.parse_request_body(test_case.request().body().unwrap()));


        println!("{}",body);
    }
}
