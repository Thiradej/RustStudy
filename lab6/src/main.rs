use rand::Rng;
use std::io;

struct Player {
    hp: i32,
    stamina: i32,
    power: i32,
    gold: i32,
}

struct Enemy {
    name: &'static str,
    hp: i32,
    power: i32,
    gold_reward: i32,
}

enum Encounter {
    Nothing,
    Meat,
    Water,
    Herb,
    Enemy(Enemy),
}


fn print_status(player: &Player) {
    println!("\n[HP: {} | Stamina: {} | Power: {} | Gold: {}]", player.hp, player.stamina, player.power, player.gold);
}


fn choose_direction() -> String {
    println!("Choose direction: N, S, E, W");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


fn random_encounter() -> Encounter {
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(0..100);
    match roll {
        0..=24 => Encounter::Nothing,
        25..=44 => Encounter::Meat,
        45..=64 => Encounter::Water,
        65..=79 => Encounter::Herb,
        _ => {
            let enemy_roll = rng.gen_range(0..100);
            if enemy_roll < 60 {
                Encounter::Enemy(Enemy {
                    name: "Rat",
                    hp: 10,
                    power: 2,
                    gold_reward: 10,
                })
            } else if enemy_roll < 90 {
                Encounter::Enemy(Enemy {
                    name: "Wolf",
                    hp: 20,
                    power: 5,
                    gold_reward: 20,
                })
            } else {
                Encounter::Enemy(Enemy {
                    name: "Boar",
                    hp: 30,
                    power: 10,
                    gold_reward: 30,
                })
            }
        }
    }
}


fn handle_encounter(player: &mut Player, encounter: Encounter) {
    match encounter {
        Encounter::Nothing => println!("Nothing here...Nigga"),
        Encounter::Meat => {
            player.hp += 5;
            if player.hp > 100 {
                player.hp = 100;
            }
            println!("You found meat! HP +5 (now {})", player.hp);
        }
        Encounter::Water => {
            player.stamina += 2;
            println!("You drank water! Stamina +2 (now {})", player.stamina);
        }
        Encounter::Herb => {
            player.power += 1;
            println!("You found an herb! Power +1 (now {})", player.power);
        }
        Encounter::Enemy(enemy) => {
            println!("A wild {} appeared!", enemy.name);
            let alive = enemy_combat(player, enemy);
            if !alive {
                println!("You were defeated...looser!");
            }
        }
    }
}

fn enemy_combat(player: &mut Player, mut enemy: Enemy) -> bool {
    while player.hp > 0 && enemy.hp > 0 {
        enemy.hp -= player.power;
        println!("You hit the {}!", enemy.name);

        if enemy.hp <= 0 {
            println!("You defeated the {}!", enemy.name);
            player.gold += enemy.gold_reward;
            println!("Gained {} gold. Total gold: {}", enemy.gold_reward, player.gold);
            return true;
        }

        player.hp -= enemy.power;
        // println!("The {} hit you!", enemy.name);
        // println!("Your HP: {}", player.hp);
    }

    player.hp > 0
}

fn main() {
    let mut player = Player {
        hp: 100,
        stamina: 50,
        power: 10,
        gold: 0,
    };

    println!("Welcome to the Adventure!");
    println!("Enter directions (N, S, E, W).");

    while player.hp > 0 && player.stamina > 0 && player.gold < 100 {
        print_status(&player);
        let dir = choose_direction();
        println!("You walked toward {}.", dir);
        player.stamina -= 1;

        let encounter = random_encounter();
        handle_encounter(&mut player, encounter);
    }

    if player.gold >= 100 {
        println!("You collected 100 gold. You win!");
    } else {
        println!("Game over. You ran out of HP or stamina.");
    }
}