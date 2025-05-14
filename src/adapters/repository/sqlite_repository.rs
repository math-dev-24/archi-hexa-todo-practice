use crate::domain::entities::todo::Todo;
use crate::domain::ports::repository::TodoRepository;

pub struct SqlRepository {}

impl TodoRepository for SqlRepository {
    fn create(&self, todo: Todo) -> Result<Todo, String> {
        unimplemented!()
    }

    fn update(&self, todo: Todo) -> Result<Todo, String> {
        unimplemented!()
    }

    fn delete(&self, id: u64) -> Result<(), String> {
        unimplemented!()
    }

    fn find_all(&self) -> Result<Vec<Todo>, String> {
        unimplemented!()
    }

    fn find_by_id(&self, id: u64) -> Result<Todo, String> {
        unimplemented!()
    }
}