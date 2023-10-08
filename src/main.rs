use std::env;

use todo_cli::ToDo;

fn main() {
    let todo: ToDo = ToDo::get_or_create();
    // println!("{:?}", todo);
    
    // cli args:
    // 1. list -> list all todos
    // 2. add <string> -> add a todo
    // 3. done <index of todo> -> mark a todo as done
    // 4. rem <index of todo> -> remove a todo (irrespective of status)
    // 5. 

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let cmd = args.get(1).unwrap();
        match &cmd[..] {
            "list" => todo.list(),
            "add" => todo.add(&args[2..]),
            _ => panic!("unexpected...."),
        };
    }
}
