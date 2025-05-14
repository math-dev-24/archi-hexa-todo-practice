use crate::Todo;

pub trait TodoRepository {
    fn create(&self, todo: Todo) -> Result<Todo, String>;
    fn update(&self, todo: Todo) -> Result<Todo, String>;
    fn delete(&self, id: u64) -> Result<(), String>;
    fn find_all(&self) -> Result<Vec<Todo>, String>;
    fn find_by_id(&self, id: u64) -> Result<Todo, String>;
}
