pub mod adapters;
pub mod domain;

use std::io;
use chrono::Datelike;
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

        let action: u8 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrée invalide");
                continue;
            }
        };

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
            2 => {
                let id = ask_id();

                let todo = match service.find_by_id(id) {
                    Ok(todo) => {
                    todo
                    }
                    Err(_) => {
                        println!("Todo introuvable");
                        break
                    }
                };
                let new_todo = Cli::update_todo(todo).unwrap();
                service.update(new_todo).unwrap();
                println!("Todo mise à jour !");
            }
            3 => {
                let id = ask_id();

                if let Ok(todo) = service.find_by_id(id) {
                    let title = todo.title;
                    service.delete(todo.id).unwrap();
                    println!("Todo supprimé {}", title);
                } else {
                    println!("Identifiant inconnu !");
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
            5 => {
                let id = ask_id();
                match service.find_by_id(id) {
                    Ok(todo) => {
                        let status = if todo.completed { "OK "} else { "NOK" };
                        println!("{} - {}", status, todo.title);
                        println!("Description : \n {}", todo.description);
                        println!("Dernière MàJ : {}", todo.updated_at);
                    }
                    Err(e) => {
                        println!("{e}")
                    }
                };
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

fn ask_id() -> u64 {
    println!("Entrez un id :");
    let mut input_id = String::new();
    io::stdin().read_line(&mut input_id).expect("Impossible de lire le contenu");

    input_id.trim().parse::<u64>().expect("Id introuvable")
}
