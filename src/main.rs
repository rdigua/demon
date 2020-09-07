use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

fn main() {
    let config: Config = toml::from_str(r#"
        ip = '127.0.0.1'
        port =8001
        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
    "#).unwrap();
    let lpstr =config.ip.clone();
    let ports=config.port.as_ref().to_owned().unwrap_or(&8081);
    let github =config.keys.github.clone();
    let travails =config.keys.travis.to_owned().unwrap_or("what".to_string());
    println!("{},{},{},{}",config.ip.clone(),config.port.as_ref().unwrap(),config.keys.github.clone(),config.keys.travis.as_ref().unwrap());
    println!("{},{},{},{}", lpstr, ports, github, travails);
    //assert_eq!(config.ip, "127.0.0.1");
    //assert_eq!(config.port, None);
    //assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    //assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
    //println!("{},{},{},{}",config.ip,config.port.as_ref().unwrap(),config.keys.github,config.keys.travis.as_ref().unwrap())
}
/*
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
    weight: Option<u8>,
}

fn main() {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    // 解析字符串到Person对象。
    let p: Person = serde_json::from_str(data).unwrap();
    println!("Please call {} at the number {}", p.name, p.phones[0]);
    
    // Person对象转为JSON字符串.
    let serialized = serde_json::to_string(&p).unwrap();
    println!("serialized = {}", serialized);
}
*/