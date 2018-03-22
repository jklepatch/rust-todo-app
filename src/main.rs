//We need to specify this to use code in other modules
mod task;
mod command;

//use std::io::{stdin, stdout, Write};
use task::{Tasks};
use command::*;

//Execution start here
fn main() {
    let start_message = "
    ##################
    # ToDo App
    ##################";
    println!("{}", start_message);

    //we use a vector of task to hold all our tasks
    //let mut tasks: Vec<Task> = Vec::new();

    let mut tasks = Tasks::new();

    loop {
        //Show to user how to select a menu
        show_usage();
        
        //Collect user input choice for menu choice
        let command = prompt_menu();

        //`as_ref()` converts the `command` String pointer to a `&str` reference
        match command.as_ref() {
            "1" => create_new_task(&mut tasks),
            "2" => see_all_tasks(&tasks),
            "3" => mark_task_as_done(&mut tasks), 
            "4" => mark_task_as_undone(&mut tasks), 
            "5" => remove_task(&mut tasks), 
            "6" => exit(0),
            _ => ()
        }
    }

}

