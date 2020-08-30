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