use serde_json;
use serde::{Serialize};
use std::{env, collections::HashMap};

#[derive(Serialize)]
struct Person {
    first_name: String,
    last_name: String,
}

#[derive(Serialize)]
struct Response {
    people: Vec<Person>,
    env_vars: HashMap<String, String>
}

fn main() {

    let r = Response {
        people: get_some_people(),
        env_vars: get_env_vars(),
    };


    println!("Content-Type: application/json\n");
    let json = serde_json::to_string(&r).unwrap();
    println!("{}", json);
}

fn get_some_people() -> Vec<Person>{
    return  vec![
        Person{
            first_name: "Bill".to_owned(),
            last_name: "Gates".to_owned(),
        },
            Person{
            first_name: "Steve".to_owned(),
            last_name: "Jobs".to_owned(),
            },
        ];
}
fn get_env_vars() -> HashMap<String, String> {
    let mut map: HashMap<String,String> = HashMap::new();

    for (key, value) in env::vars() {
        map.insert(key, value);
    }
    return map;
}
