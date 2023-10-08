use colored::*;
use std::{env::{self}, fs::OpenOptions, io::Read};
use log::info;

#[derive(Debug)]
pub struct ToDo {
    pub todo: Vec<String>,
    pub file_path: String,
}

impl ToDo {
    pub fn get_or_create() -> ToDo {
        // 1. Check env var
        let file_path: String = match env::var("TODO_PATH") {
            Ok(value) => value,
            Err(e) => {
                info!("Unable to find TODO_PATH env variable. Error: {}", e);
                let home_path = match env::var("HOME") {
                    Ok(p) => p,
                    Err(_) => "/tmp".to_string(),
                };
                let path = format!("{}/.todo", home_path);
                env::set_var("TODO_PATH", &path);
                path
            },
        };

        // 2. Check file and read to todo vec
        let mut file_contents = String::new();

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_path)
            .expect(&format!("Unable to create/load file: {}", file_path));

        file.read_to_string(&mut file_contents).unwrap();

        info!("Contents of file {} is: {}", file_path, file_contents);

        // load file_content into vec
        let todo: Vec<String> = file_contents.lines().map(str::to_string).collect();

        ToDo { todo, file_path }
    }

    pub fn list(&self) {
        for (idx, task) in self.todo.iter().enumerate() {
            // split "[  ] "(not done) or "[x] "(done)
            // append status at end
            let mut data = String::new();
            let status = &task[..4];
            let content = &task[4..];
            if status == "[ ] " {
                data = format!("{}. {}", idx, content.bold());
            } else if status == "[x] " {
                data = format!("{}. {}", idx, content.strikethrough());
            }
            println!("{}", data);
        }
    }

    pub fn add(&self, tasks: &[String]) {
        for task in  tasks {
            print!("{} ", task);
        }
    }
}
