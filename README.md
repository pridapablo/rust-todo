This is my first Rust project. It's a simple command line tool for creating and managing a todo list.

The code was written following [this](https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/) tutorial.

Note: in the initial implementation the todo list is stored in a file for long term persistence. The first extension to the project will be to use a database instead.

## Usage

### Add a task

```rust
$ cargo run add "Buy milk"
```

## Roadmap:

- [x] Add a task
- [ ] List tasks
- [ ] Mark a task as complete
- [ ] Delete a task
- [ ] Store tasks in a file
- [ ] Store tasks in a database
- [ ] Add a due date to a task
- [ ] Add a priority to a task
- [ ] Add a project to a task
- [ ] List tasks by due date
- [ ] List tasks by priority
- [ ] List tasks by project
- [ ] Add a command to list all tasks
- [ ] Add a command to list all tasks by due date
- [ ] Add a command to list all tasks by priority
- [ ] Add a command to list all tasks by project
- [ ] Add a command to list all tasks by due date and priority
