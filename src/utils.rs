use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub age: u32
}


pub fn write_json_file(file_path: &str, users: &Vec<User>) -> Result<(), std::io::Error> {
    // Serialize the Rust struct into a JSON string
    let json_data = serde_json::to_string(users)?;

    // Open the file for writing
    let mut file = File::create(file_path)?;

    // Write the JSON string to the file
    file.write_all(json_data.as_bytes())?;

    Ok(())
}



pub fn read_json_file(file_path_is : &str) {
    println!("Opening file {}", file_path_is);

    let file_path = file_path_is;
    let mut file = File::open(file_path).expect("Failed to open file");

    // Read the file contents into a string
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)
        .expect("Failed to read file");

    // Deserialize the JSON string into a Rust struct
    let user: Vec<User> = serde_json::from_str(&file_content)
        .expect("Failed to deserialize JSON");

    // Now you can work with the deserialized data
    for u in &user {
        println!("Name: {}, Age: {}", u.name, u.age);
    }
}