use std::cell::RefCell;
use crate::domain::entities::todo::Todo;
use crate::domain::ports::repository::TodoRepository;

pub struct MockTodoRepository {
    todos: RefCell<Vec<Todo>>
}

impl MockTodoRepository {
    pub fn new() -> Self {
        Self {
            todos: RefCell::new(Vec::new())
        }
    }
}

impl TodoRepository for MockTodoRepository {
    fn create(&self, todo: Todo) -> Result<Todo, String> {
        let mut todos = self.todos.borrow_mut();
        let new_todo = Todo {
            id: todos.len() as u64 + 1,
            ..todo
        };
        todos.push(new_todo.clone());
        Ok(new_todo)
    }

    fn update(&self, todo: Todo) -> Result<Todo, String> {
        let mut todos = self.todos.borrow_mut();
        if let Some(index) = todos.iter().position(|t| t.id == todo.id) {
            todos[index] = todo.clone();
            Ok(todo)
        } else {
            Err("Todo not found".to_string())
        }
    }

    fn delete(&self, id: u64) -> Result<(), String> {
        let mut todos = self.todos.borrow_mut();
        if let Some(index) = todos.iter().position(|t| t.id == id) {
            todos.remove(index);
            Ok(())
        } else {
            Err("Todo not found".to_string())
        }
    }

    fn find_all(&self) -> Result<Vec<Todo>, String> {
        Ok(self.todos.borrow().clone())
    }

    fn find_by_id(&self, id: u64) -> Result<Todo, String> {
        let todos = self.todos.borrow();
        todos
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or_else(|| "Todo not found".to_string())
    }
}