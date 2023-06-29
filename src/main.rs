// Imports
use std::{collections::HashMap, fmt::format};

// All variables are immutable by default We can opt to make it mutable by using
// "mut"

// Main function
fn main() {
    // CLI input binding code:
    let action = std::env::args().nth(1).expect("Please specify an action");
    // considering that std::env::args() is an iterator we use .nth to access
    // values .expect is a method for the Option enum that will return the value
    // or panic with that return msg
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("Recieved action: {:?}, and item: {:?}", action, item);

    // We instanciate the class and initialize the map. The todo instance is mutable
    let mut todo = Todo{
        map: HashMap::new(),
    };

    if action == "add" {
        // we call the insert method
        todo.insert(item);
        // we match the result type returned from the save fn
        match todo.save() {
            Ok(_) => println!("Todo Saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}

// We can implement our own type for the todo list
struct Todo {
    // Using the built in HashMap for key-value pairs map is our field
    // HashMap<String, bool> means that keys will be strings and values booleans
    map: HashMap<String, bool>
}

// impl Todo to add methods to the struct Their first parameter is ALWAYS self
impl Todo {
    // we declare a function (method) It takes two arguments, the first is self
    // (a mutable pointer to the location where value is stored and the key)
    // NOTE: this is a borrow since we don't "own" self (use &)
    fn insert(&mut self, key: String) {
        // insert a new item into the hash map passing true as value
        self.map.insert(key, true);
        // insert is a built in method
    }    

    // We will write to a file to store todo's
    fn save(self) -> Result<(), std::io::Error>
    // Here, -> is for typing the return of the function (it returns a result for error handling)
     {
        let mut content = String::new();

        for (k, v) in self.map {
            // we iterate over the hash map and format each string, separating
            // key and value with a tabl char (\t) and each line with a new line
            // (\n)
            let record = format!("{}\t{}\n",k,v);
            content.push_str(&record)
        }
        // We persist the data in the db.txt file
        std::fs::write("db.txt", content)
    }
}

// The ownership system has three rules: Each value in Rust has a variable: its
// owner. There can only be one owner at a time for each value. When the owner
// goes out of scope, the value will be dropped.