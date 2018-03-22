use std::fmt;

#[derive(Debug)]
pub struct Task {
    id: i32,
    name: String,
    description: String,
    author: String,
    done: bool
}

impl Task {
    pub fn new(name: String, description: String, author: String) -> Task {
        Task {
            id: 0,
            name, 
            description, 
            author, 
            done: false
        }
    }

    pub fn mark_as_done(&mut self) {
        self.done = true;
    }

    pub fn mark_as_undone(&mut self) {
        self.done = false;
    }
}

pub struct Tasks {
    count: i32,
    next_id: i32,
    data: Vec<Task>
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {
            count: 0,
            next_id: 0,
            data: Vec::new()
        }
    }

    pub fn get_task(&mut self, task_id: &i32) -> Option<&mut Task> {
        for task in self.data.iter_mut() {
            if task.id == *task_id {
                return Some(task);
            }
        }

        None
    }

    pub fn add_task(&mut self, mut task: Task) -> i32 {
        task.id = self.next_id; 
        self.data.push(task);
        self.count += 1;
        self.next_id += 1;
        return self.next_id - 1;
    }

    pub fn remove_task(&mut self, task_id: &i32) -> Option<Task> {
        match self.data.iter().position(|task| task.id == *task_id) {
            Some(i) => {
                self.count -= 1;
                return Some(self.data.remove(i));
            },
            None => return None
        }
    }
}

impl fmt::Debug for Tasks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tasks: {:#?}", self.data) 
    }
}

#[cfg(test)]
mod tests {
    use task::{Tasks, Task};

    #[test]
    fn task_properly_initialized() {
        let task = Task::new(
            "Buy milk".into(),
            "Buy 5 litters of milk of brand X".into(), 
            "Joe".into() 
        );

        assert_eq!(task.id, 0);
        assert_eq!(task.name, "Buy milk");
        assert_eq!(task.description, "Buy 5 litters of milk of brand X");
        assert_eq!(task.author, "Joe");
        assert_eq!(task.done, false);
    }

    #[test]
    fn add_task_1() {
        let mut tasks = Tasks::new();

        let task = Task::new(
            "Buy milk".into(), 
            "Buy 5 litters of milk of brand X".into(), 
            "Joe".into() 
        );
        tasks.add_task(task);

        match tasks.get_task(&0) {
            Some(task) => {
                assert_eq!(task.name, "Buy milk");
                assert_eq!(task.description, "Buy 5 litters of milk of brand X");
                assert_eq!(task.author, "Joe");
            },
            None => assert!(false)
        }
    }

    #[test]
    fn add_task_2() {
        let mut tasks = Tasks::new();

        let task_1 = Task::new(
            "Buy milk".into(), 
            "Buy 5 litters of milk of brand X".into(), 
            "Joe".into() 
        );
        let task_2 = Task::new(
            "Buy almond".into(), 
            "Buy 1 kilogram of almond".into(), 
            "Joe".into() 
        );
        tasks.add_task(task_1);
        tasks.add_task(task_2);

        match tasks.get_task(&1) {
            Some(task) => {
                assert_eq!(task.name, "Buy almond");
                assert_eq!(task.description, "Buy 1 kilogram of almond");
                assert_eq!(task.author, "Joe");
            },
            None => assert!(false)
        }
    }

    #[test]
    fn remove_task_1() {
        let mut tasks = Tasks::new();

        let task_1 = Task::new(
            "Buy milk".into(), 
            "Buy 5 litters of milk of brand X".into(), 
            "Joe".into() 
        );
        let task_2 = Task::new(
            "Buy almond".into(), 
            "Buy 1 kilogram of almond".into(), 
            "Joe".into() 
        );
        let task_1_id = tasks.add_task(task_1);
        tasks.add_task(task_2);
        assert_eq!(tasks.count, 2);

        match tasks.remove_task(&task_1_id) {
            Some(removed_task) => {
                assert_eq!(removed_task.name, "Buy milk");
                assert_eq!(tasks.count, 1);
            },
            None => assert!(false),
        }
    }

    #[test]
    fn tasks_start_with_no_task() {
        let tasks = Tasks::new();
        assert_eq!(tasks.count, 0);
    }

    #[test]
    fn task_count_increments_after_adding_task_1() {
        let mut tasks = Tasks::new();
        let task = Task::new(
            "Buy milk".into(), 
            "Buy 5 litters of milk of brand X".into(), 
            "Joe".into() 
        );
        tasks.add_task(task);
        assert_eq!(tasks.count, 1);
    }

    #[test]
    fn task_count_increments_after_adding_task_2() {
        let mut tasks = Tasks::new();
        let task1 = Task::new(
            "Buy milk".into(), 
            "Buy 5 litters of milk of brand X".into(), 
            "Joe".into() 
        );
        let task2 = Task::new(
            "Buy milk".into(), 
            "Buy 5 litters of milk of brand X".into(), 
            "Joe".into() 
        );
        tasks.add_task(task1);
        tasks.add_task(task2);
        assert_eq!(tasks.count, 2);
    }
}
