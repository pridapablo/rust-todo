// Imports
use std::{collections::HashMap, io::Read};
use std::str::FromStr;

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

    // We instanciate the class and initialize the map. The todo instance is
    // mutable (we use the new function in the class to avoid overwriting the file)
    // THIS WOULD OVERWRITE:
    // let mut todo = Todo{
    //     map: HashMap::new(),
    // };
    let mut todo = Todo::new().expect("Initialisation of db failed");

    // Adding a task to the Todo list
    if action == "add" {
        // we call the insert method
        todo.insert(item);
        // we match the result type returned from the save fn
        match todo.save() {
            Ok(_) => println!("Todo Saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }

    // Crossing out a task from the list
    else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            // Match returns the actual item, so we save it
            Some(_) => match todo.save() {
                // The save method returns a Result. We can error handle here
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
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
    // This is not a method since self is not the first argumen. This is
    // necesary since our program shouldn't overwrite the map, it should update
    // it. The fn returns the todo struct or an error Functional programming
    // approach. NOTE: a "for loop" alternative is available in the tutorial
    // followed
    // (https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/)
    fn new() -> Result<Todo, std::io::Error>{
        // f is the file. Here we define that we will create the file if it
        // doesn't exist let us read and write to the file.
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.txt")?;

        // Empty string that will store the whole file
        let mut content = String::new();
        // Reads the file and appends to the content string
        f.read_to_string(&mut content)?;

        // Convert the string to a hash map
        let map: HashMap<String, bool> = content
            // create an iterator over each line of a string (we go line by
            // line)
            .lines()
            // Split the line with tab and convert the iterator into the
            // relevant collection
            .map(|line| line.splitn(2,"\t").collect::<Vec<&str>>())
            // Transform the result into a tuple
            .map(|v| (v[0], v[1]))
            // Convert the two elements of the tuple into a String and a boolean
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            // Collect into hashmap (type infered from binding declaration)
            .collect();
        // Return the todo with the map
        Ok(Todo { map })
    }

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
    // Here, -> is for typing the return of the function (it returns a result
    // for error handling)
     {
        let mut content = String::new();

        for (k, v) in self.map {
            // we iterate over the hash map and format each string, separating
            // key and value with a tabl char (\t) and each line with a new line
            // (\n)
            let record = format!("{}\t{}\n",k,v);
            content.push_str(&record)
        }
        // We persist the data in the db.txt file (using filesystem a.k.a. fs)
        std::fs::write("db.txt", content)
    }

    // Method to complete a task
    // return type is an empty option
    // the function returns the result of the match expression (either an empty Some() or None)
    fn complete(&mut self, key: &String) -> Option<()>{
        // The .get_mut method gives a mutable ref to the value of the key (None if not found)
        match self.map.get_mut(key) {
            // We need to de-reference the value (pointer)
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

// The ownership system has three rules: Each value in Rust has a variable: its
// owner. There can only be one owner at a time for each value. When the owner
// goes out of scope, the value will be dropped.