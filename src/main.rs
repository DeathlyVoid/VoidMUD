use rand::prelude::*;
use serde_derive::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
struct GameData {
    player_hp: i32,
    coins: i32,
    rounds: i32,
    monster: bool,
    monster_health: i32,
    monster_damage: i32,
    dodge_correctly: bool,
    charge_up_times: u32,
    gameover: bool,
}

fn main() {
    let mut game_data = load_game_data(); // Load from file or create new
    let mut coins = game_data.coins;
    let mut gameover: bool = false;
    let mut rounds = 0; // Declare `rounds` outside the loop to avoid redeclaration
    loop {
            // Reset monster_damage at the start of each encounter
    game_data.monster_damage = 10;
        let charge_up_times = rand::thread_rng().gen_range(1..=10); // Initialize charge_up_times
        let player_damage: i32 = rand::thread_rng().gen_range(40..=100);
        if !game_data.monster {
            println!("Monster is mustering a powerful attack!!!!!");
        }

        for _ in 0..charge_up_times {
            println!("Monster is charging up");
            game_data.monster_damage += 2;
        }

        println!("The monster is attacking with {} damage!!", game_data.monster_damage);
           
        dodge(&mut game_data, player_damage);
        roundcheck(&mut game_data, &mut coins, &mut gameover);

        if gameover == true {
            break;
        }

            // Save game data after each encounter
            save_game_data(&game_data);

            // Prompt for action or exit
            let mut input = String::new();
            println!("Continue? (y/n)");
            std::io::stdin().read_line(&mut input).unwrap();
            if input.trim() != "y" {
                return; // Exit the entire program if player doesn't want to continue
            }

            rounds += 1;
        }
    }

    fn dodge(game_data: &mut GameData, player_damage: i32) {
        let dodge_correctly = rand::thread_rng().gen_bool(0.5); // Initialize dodge_correctly
        if dodge_correctly {
            println!("You dodged the monster's {} damage and counterattack!", game_data.monster_damage);
        game_data.monster_health -= player_damage;
        println!("You dealt {} damage to the monster! It's health is {}", player_damage, game_data.monster_health)
        } else {
            game_data.player_hp -= game_data.monster_damage;
            println!("You were hit by {} damage from the monster!!! Health is now {}", game_data.monster_damage, game_data.player_hp);
        }

    }

    fn roundcheck(game_data: &mut GameData, coins: &mut i32, gameover: &mut bool) {
                    // Check for game over
                    if game_data.player_hp <= 0 {
                        println!("Game Over! You have been defeated... You lasted {} rounds.", game_data.rounds);
                        *game_data = reset_game_data(); // Reset the game data
                        game_data.coins = *coins; // Assign the updated coins value
                        *gameover = true;
                        save_game_data(&game_data); // Save the reset game data
                    }
                
                    if game_data.monster_health <= 0 {
                        *coins += 50;
                        println!("Victory! You beat the monster! Here are some coins for defeating the monster. Your new total of coins is {}", coins);
                        *game_data = reset_game_data(); // Reset the game data except for coins
                        game_data.coins = *coins; // Assign the updated coins value
                        *gameover = true;
                        save_game_data(&game_data); // Save the reset game data
                    }
        
                game_data.rounds += 1; // Increment rounds
    }

fn reset_game_data() -> GameData {
    GameData {
        monster_health: 250,
        player_hp: 100, // Reset the player's health to 100
        rounds: 0,
        monster: true,
        monster_damage: 10,
        dodge_correctly: false, // Initialize dodge_correctly to false
        charge_up_times: 0, // Initialize charge_up_times to 0
        coins: 0,
        gameover: false,
    }
}

fn save_game_data(data: &GameData) {
    let serialized = serde_json::to_string(&data).unwrap();
    let mut file = File::create("save_data.json").unwrap();
    file.write_all(serialized.as_bytes()).unwrap();
}

fn load_game_data() -> GameData {
    let file_result = File::open("save_data.json");

    if let Ok(mut file) = file_result {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            if let Ok(data) = serde_json::from_str(&contents) {
                return data;
            }
        }
    }

    // Handle empty or invalid file by returning default data
    println!("No valid save data found, starting new game!");
    reset_game_data()
}