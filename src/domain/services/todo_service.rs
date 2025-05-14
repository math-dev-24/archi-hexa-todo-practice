use crate::domain::entities::todo::Todo;
use crate::domain::ports::repository::TodoRepository;


pub struct TodoService<'a, T: TodoRepository> {
    repository: &'a T,
}

impl<'a, T: TodoRepository> TodoService<'a, T> {
    pub fn new(repository: &'a T) -> Self {
        Self { repository }
    }

    pub fn create(&self, todo: Todo) -> Result<Todo, String> {
        self.repository.create(todo)
    }

    pub fn update(&self, todo: Todo) -> Result<Todo, String> {
        self.repository.update(todo)
    }

    pub fn delete(&self, id: u64) -> Result<(), String> {
        self.repository.delete(id)
    }

    pub fn find_all(&self) -> Result<Vec<Todo>, String> {
        self.repository.find_all()
    }

    pub fn find_by_id(&self, id: u64) -> Result<Todo, String> {
        self.repository.find_by_id(id)
    }
}
