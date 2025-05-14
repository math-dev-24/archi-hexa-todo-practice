pub mod adapters;
pub mod domain;

use std::io;
use adapters::repository::json_repository::JsonRepository;
use domain::services::todo_service::TodoService;
use domain::entities::todo::Todo;
use adapters::cli::menu::Cli;


fn main() {
    let repository = JsonRepository::new("data/todos.json".to_string());
    let service = TodoService::new(&repository);

    println!("\n === Gestionnaire de tâches === ");
    println!("Vous pouvez effectuer plusieurs actions :");
    println!("   1. Créer une nouvelle tâche");
    println!("   2. Mettre à jour une tâche");
    println!("   3. Supprimer une tâche");
    println!("   4. Afficher toutes les tâches");
    println!("   5. Rechercher une tâche par ID");
    println!("   6. Quitter");

    loop{
        println!("\n Choisissez une action : ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Impossible de lire le contenu");

        let action: u8 = input.trim().parse().expect("Doit être un nombre !");

        if action > 6 || action < 1 {
            println!("\n Doit être compris entre 1 et 6 !");
            continue
        }

        match action {
            1 => {
                let new_todo = Cli::create_todo().unwrap();
                match service.create(new_todo) {
                    Ok(_) => {
                        println!("Todo ajoutée !")
                    }
                    Err(e) => {
                        println!("{e}")
                    }
                }
            }
            4 => {
                match service.find_all() {
                    Ok(todos) => {
                        for todo in todos {
                            let state = if todo.completed { "OK" } else { "NOK" };
                            println!("{} - {}: {} ", state, todo.id, todo.title);
                        }
                    }
                    Err(e) => {
                        println!("{e}")
                    }
                }
            }
            6 => {
                break
            }
            _ => {
                unimplemented!()
            }
        }
    }
    println!("Au revoir !")
}
