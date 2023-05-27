use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn typed_example(file_path: &str) -> Result<()> {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let p: Person = serde_json::from_str(data)?;

    // 写文件
    std::fs::write(
        file_path,
        serde_json::to_string_pretty(&p).unwrap(),
    ).unwrap();

    println!("Please call {} at the number {}", p.name, p.phones[0]);
    Ok(())
}

fn read_and_write(read_path: &str, write_path: &str) -> Result<()> {
    let json_str = std::fs::read_to_string(&read_path).unwrap();
    let mut p: Person = serde_json::from_str::<Person>(&json_str).unwrap();

    p.name = "JiangHan".to_string();
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    std::fs::write(
        write_path,
        serde_json::to_string_pretty(&p).unwrap(),
    ).unwrap();
     Ok(())
}

fn main() -> Result<()> {
    typed_example("/tmp/rust_data.json");
    read_and_write("/tmp/rust_data.json", "/tmp/rust_data_w.json");
    Ok(())
}
