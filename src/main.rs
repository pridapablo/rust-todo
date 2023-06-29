// Imports
use std::collections::HashMap;

// All variables are immutable by default
// 

// Main function
fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    // considering that std::env::args() is an iterator we use .nth to access
    // values .expect is a method for the Option enum that will return the value
    // or panic with that return msg
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);
}

// We can implement our own type for the todo list
struct Todo {
    // Using the built in HashMap for key-value pairs
    // map is our field
    // HashMap<String, bool> means that keys will be strings and values booleans
    map: HashMap<String, bool>
}

// impl Todo to add methods to the struct
// Their first parameter is ALWAYS self
impl Todo {
    // we declare a function (method)
    fn insert(&mut self, key: String) {
        // insert a new item into the hash map
        // passing true as value
        self.map.insert(key, true);
        // insert is a built in method
    }    
}