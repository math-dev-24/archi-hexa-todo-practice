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

        let todo = Todo::new(
            title.trim().to_string(),
            description.trim().to_string()
        );
        Ok(todo)
    }

    pub fn update_todo(todo: Todo) -> Result<Todo, String> {

        let mut new_todo: Todo = todo.clone();

        println!("Modifier le titre {}", new_todo.title);

        let mut title = String::new();
        io::stdin().read_line(&mut title).expect("Une erreur est survenue");

        let mut description = String::new();
        io::stdin().read_line(&mut description).expect("Une erreur est survenue");

        new_todo.set_title(title);
        new_todo.set_description(description);

        Ok(new_todo)
    }
}