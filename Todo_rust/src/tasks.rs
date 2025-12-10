use csv::{Reader, Writer};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub task_name: String,
    pub task_description: String,
    pub task_complete: String,
}

pub fn load_tasks() -> Vec<Task> {
    let mut tasks: Vec<Task> = vec![];

    let mut file = File::open("tasks.csv")
        .unwrap_or_else(|_| File::create("tasks.csv").unwrap());

    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_ok() {
        let mut reader = Reader::from_reader(contents.as_bytes());
        for result in reader.deserialize() {
            let task: Task = result.unwrap();
            tasks.push(task);
        }
    }

    tasks
}

pub fn save_tasks(tasks: &[Task]) {
    let file = File::create("tasks.csv").expect("Unable to create file");
    let mut writer = Writer::from_writer(file);

    for task in tasks {
        writer.serialize(task).expect("Unable to write task");
    }
}
