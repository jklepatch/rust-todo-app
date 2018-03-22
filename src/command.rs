//mod task; //We need to specify this to use code contained in the `task.rs` file
use task::{Task, Tasks}; //our Task struct
use std::io::{stdin, stdout, Write}; //Need to import these for prompting user and echoing to terminal
use std::process::{exit as process_exit};

pub fn create_new_task(tasks: &mut Tasks) {
    let new_task = prompt_new_task();
    println!("New task created: {:?}", new_task);
    tasks.add_task(new_task);
}

pub fn see_all_tasks(tasks: &Tasks) {
    println!("see all tasls");
    println!("All tasks created: {:#?}", tasks);
}

pub fn mark_task_as_done(tasks: &mut Tasks) {
    let mut task_id = String::new();
    prompt("Task id: ", &mut task_id);

    let task_id_i32: i32 = task_id.parse().unwrap();

    match tasks.get_task(&task_id_i32) {
        Some(task) =>  {
            task.mark_as_done();
            println!("Task {} was marked as done", task_id_i32);
        },
        None => println!("Task {} does not exist!", task_id_i32),
    }
}

pub fn mark_task_as_undone(tasks: &mut Tasks) {
    let mut task_id = String::new();
    prompt("Task id: ", &mut task_id);

    let task_id_i32: i32 = task_id.parse().unwrap();

    match tasks.get_task(&task_id_i32) {
        Some(task) =>  {
            task.mark_as_undone();
            println!("Task {} was marked as undone", task_id_i32);
        },
        None => println!("Task {} does not exist!", task_id_i32),
    }
}

pub fn remove_task(tasks: &mut Tasks) {
    let mut task_id = String::new();
    prompt("Task id: ", &mut task_id);

    let task_id_i32: i32 = task_id.parse().unwrap();

    match tasks.remove_task(&task_id_i32) {
        Some(_removed_task) =>  {
            println!("Task {} was removed", task_id_i32);
        },
        None => (), //This will never be called because remove_task() might panic
    }
}

pub fn exit(code: i32) {
    println!("Exiting ToDo App...");
    process_exit(code);
}

pub fn show_usage() {
    //Multi-line strings written just like single-line ones"
    let menu = "What do you want to do?
    1: Create New Task
    2: See All Tasks
    3: Mark Task As Done
    4: Mark Task As Undone
    5: Remove Task
    6: Exit";
    println!("{}", menu);
}

fn prompt_new_task() -> Task {
    let mut task_name = String::new();
    let mut task_desc = String::new();
    let mut task_author = String::new();

    prompt("Task name: ", &mut task_name);
    prompt("Task description: ", &mut task_desc);
    prompt("Task author: ", &mut task_author);

    Task::new(
        task_name.clone(), 
        task_desc.clone(), 
        task_author.clone()
    )
}

pub fn prompt_menu() -> String {
    let mut command = String::new();
    prompt("Your choice: ", &mut command);
    command
}

fn prompt(name: &str, s: &mut String) {
    print!("{}", name);

    let _ = stdout().flush();
    stdin().read_line(s).expect("You did not enter a proper input");

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }

    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
}
