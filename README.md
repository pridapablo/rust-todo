# Rust TODO List
This is my first Rust project. It's a simple command line tool for creating and managing a todo list.

The initial code was written following [this](https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/) tutorial.

Note: in the initial implementation the todo list is stored in a file for long term persistence. The first extension to the project will be to use a database instead.

## Usage

### Add a task

```rust
$ cargo run add "Learn Rust"
```

### Mark a task as complete

```rust
$ cargo run complete "Learn Rust"
```

## Roadmap (a.k.a. a todo list for making a todo list)

Note: The approach to cross off items from this list is to use the text in the roadmap as an initial guide. As I learn more about Rust I will update the roadmap to reflect the best way to implement the features and I will add the resources I used to learn about them.

- [x] Add a task
- [x] Persist tasks
- [x] Mark a task as complete
- [ ] List tasks
      Iterate over your HashMap and print out each key-value pair. Don't forget to add a new command for this in your main function.
- [ ] Delete a task
      You can use the HashMap::remove() function to remove a key-value pair from the HashMap. Add a new command for this in your main function.
- [ ] Store tasks in a database
      This will be a major step. Consider using a SQLite database for simplicity. The rusqlite crate can help with this. Replace your current file-based persistence with database operations.
- [ ] Add a due date to a task
      You will need to change your HashMap to associate a task with more than just a boolean. Consider making it a HashMap<String, Task> where Task is a struct with fields for completion status, due date, etc.
- [ ] Add a description to a task
- [ ] Add a priority to a task
      Similar to adding a due date, you will add more fields to your Task struct and update your functions to handle these fields.
- [ ] Add a project to a task
- [ ] List tasks by due date
- [ ] List tasks by priority
- [ ] List tasks by project
      For these, you might find it helpful to have a Vec<Task> in addition to or instead of your HashMap. This will allow you to sort your tasks by various fields. The sort_by() method of a Vec will be useful.
- [ ] Add a command to edit a task
- [ ] Add a GUI using web assembly
      This will be a huge step. Consider using the yew crate to build a web app. You will need to replace your current command line interface with a web interface. You will also need to replace your current persistence with a web API. The actix-web crate can help with this.
