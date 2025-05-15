
#[cfg(test)]
mod todo_service_test {
    use crate::adapters::repository::mock_repository::MockTodoRepository;
    use crate::domain::entities::todo::Todo;
    use crate::domain::services::todo_service::TodoService;


    fn create_test_todo() -> Todo {
        Todo {
            id: 0,
            title: "test to todo".to_string(),
            completed: false,
            description: "Longue description".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }


    #[test]
    fn test_create_todo() {
        let repo = MockTodoRepository::new();
        let service = TodoService::new(&repo);

        let new_todo = create_test_todo();

        let created = service.create(new_todo).unwrap();

        assert_eq!(created.id, 1);
        assert_eq!(created.title, "test to todo");
        assert_eq!(created.completed, false);

        let all_todo = service.find_all().unwrap();
        assert_eq!(all_todo.len(), 1);
    }

    #[test]
    fn test_find_by_id() {
        let repo = MockTodoRepository::new();
        let service = TodoService::new(&repo);

        let new_todo = create_test_todo();

        let created = service.create(new_todo).unwrap();
        let found = service.find_by_id(created.id).unwrap();

        assert_eq!(found.id, created.id);
        assert_eq!(found.title, created.title);
        assert_eq!(found.description, created.description);
    }

    #[test]
    fn test_update_todo() {
        let repo = MockTodoRepository::new();
        let service = TodoService::new(&repo);

        let new_todo = create_test_todo();
        let created = service.create(new_todo).unwrap();

        let mut update_todo = created.clone();

        update_todo.title = "test to update".to_string();

        let updated = service.update(update_todo).unwrap();
        assert_eq!(updated.title, "test to update");

    }

    #[test]
    fn test_delete_todo() {
        let repo = MockTodoRepository::new();
        let service = TodoService::new(&repo);

        let new_todo = create_test_todo();
        let created = service.create(new_todo).unwrap();
        
        assert_eq!(service.find_all().unwrap().len(), 1);
        
        service.delete(created.id).unwrap();
        
        assert_eq!(service.find_all().unwrap().len(), 0);

    }
}