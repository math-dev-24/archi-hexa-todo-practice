use std::io;
use crate::domain::entities::todo::Todo;

pub struct Cli {}

impl Cli {
    pub fn create_todo() -> Result<Todo, String> {
        println!("Entrez le titre de la tâche :");
        
        let mut title = String::new();
        io::stdin().read_line(&mut title)
            .expect("Une erreur est survenu lors de la récupération du titre");

        
        println!("Entrez la description de la tâche :");
        
        let mut description = String::new();
        io::stdin().read_line(&mut description)
            .expect("Une erreur est survenue lors de la récupération de la description");

        let id = chrono::Utc::now().timestamp_millis() as u64;
        
        let todo = Todo::new(
            id,
            title.trim().to_string(),
            description.trim().to_string()
        );
        
        Ok(todo)

    }
}