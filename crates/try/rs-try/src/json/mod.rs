use serde_json::{json, Value};
use std::{collections::HashMap, error::Error, fs::File, io::BufReader};

type JsonMap = HashMap<String, Value>;

/*
    TODO X:
        1. 解析 json 文件

*/
pub fn parse_json(path: &str) -> Result<Value, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // parse:
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_file() {
        let path = "fixtures/cookiecutter.json";

        let data = parse_json(path);

        println!("json file: {:?}", data);

        // 输出字典结构:
        for item in data {
            match item {
                Value::Null => {},
                Value::Bool(_) => {},
                Value::Number(_) => {},
                Value::String(_) => {},
                Value::Array(_) => {},
                Value::Object(map) => {
                    for (k, v) in map {
                        println!("\t{}: {}", k, v);
                    }
                },
            }
        }
    }

    #[test]
    fn it_works() {
        // The type of `john` is `serde_json::Value`
        let john = json!({
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        });

        println!("first phone number: {}", john["phones"][0]);

        // Convert to a string of JSON and print it out
        println!("{}", john.to_string());
    }
}
