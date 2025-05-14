use std::fs;
use std::fs::File;
use std::io::Write;
use serde_json::{from_str, to_string_pretty};
use crate::domain::entities::todo::Todo;
use crate::domain::ports::repository::TodoRepository;

pub struct JsonRepository {
    path: String
}


impl JsonRepository {
    pub fn new(path: String) -> Self {
        JsonRepository { path }
    }

    fn load_todos(&self) -> Result<Vec<Todo>, String> {
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                if content.is_empty() {
                    Ok(Vec::new())
                } else {
                    from_str(&content)
                        .map_err(|e|
                            format!("Erreur lors de la lecture du fichier JSON: {}", e)
                        )
                }
            }
            Err(_) => {
                    Ok(Vec::new())
            }
        }
    }

    fn save_todos(&self, todos: &[Todo]) -> Result<(), String> {
        let json = to_string_pretty(todos)
            .map_err(|e| format!("Erreur lors de la sérialisation en JSON: {}", e))?;

        File::create(&self.path)
            .and_then(|mut file| file.write_all(json.as_bytes()))
            .map_err(|e| format!("Erreur lors de l'écriture dans le fichier: {}", e))
    }

}


impl TodoRepository for JsonRepository {
    fn create(&self, todo: Todo) -> Result<Todo, String> {
        let mut todos = self.load_todos()?;
        todos.push(todo.clone());
        self.save_todos(&todos)?;
        Ok(todo)
    }

    fn update(&self, todo: Todo) -> Result<Todo, String> {
        let mut todos = self.load_todos()?;

        if let Some(index) = todos.iter().position(|t| t.id == todo.id) {
            todos[index] = todo.clone();
            self.save_todos(&todos)?;
            Ok(todo)
        } else {
            Err(format!("Todo avec l'ID {} non trouvé", todo.id))
        }
    }

    fn delete(&self, id: u64) -> Result<(), String> {
        let mut todos = self.load_todos()?;

        if let Some(index) = todos.iter().position(|t| t.id == id) {
            todos.remove(index);
            self.save_todos(&todos)?;
            Ok(())
        } else {
            Err(format!("Todo avec l'ID {} non trouvé", id))
        }
    }

    fn find_all(&self) -> Result<Vec<Todo>, String> {
        self.load_todos()
    }

    fn find_by_id(&self, id: u64) -> Result<Todo, String> {
        let todos = self.load_todos()?;

        todos.iter()
            .find(|t| t.id == id)
            .cloned()
            .ok_or_else(|| format!("Todo avec l'ID {} non trouvé", id))
    }
}
