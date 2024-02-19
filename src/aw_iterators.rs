// implement a struct todo_list that is a hashmap of String->bool
// implement iterator for todo_list, the next method should return a String with the next task
// while also removing all the tasks that are done from the hashmap
#![allow(dead_code)]

use std::collections::HashMap;

pub struct TodoList {
    list: HashMap<String, bool>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList {
            list: HashMap::new(),
        }
    }

    pub fn add(&mut self, task: String) {
        self.list.insert(task, false);
    }
}

impl Iterator for TodoList {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut next_task = None;
        for (task, done) in self.list.iter() {
            if !*done {
                next_task = Some(task.clone());
                break;
            }
        }
        if let Some(task) = &next_task {
            self.list.remove(task);
        }
        next_task
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_list() {
        let mut todo_list = TodoList::new();
        todo_list.add("task1".to_string());
        todo_list.add("task2".to_string());
        todo_list.list.insert("task1".to_string(), true);
        todo_list.list.insert("task2".to_string(), true);
        assert_eq!(todo_list.next(), None);
    }
}