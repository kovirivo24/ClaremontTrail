#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

// mod events;
mod player; // iimporting the player module
mod room; // importing the room module
use player::item::item::item; // since player imports item, we don't have to re-import it but jsut "use" it
use player::player::Player;

// use crate::room::Room; //Mod player and struct player is different, so we are using Player structure
use room::events::events::events;
use room::Room::Room;
// Same thing but for the room mod

// pub mod item;
pub enum School {
    HarveyMudd,
    Pomona,
    Pitzer,
    Scripps,
    Cmc,
}

//Helps keep track of global variables
struct GameState {
    currentSchool: School,
    schoolRoom: u8,
    intro: bool,
}

//Helper function to quickly adjust the players stats according to the event they chose
fn statAdjuster(p: &mut Player, e: &events) {
    p.health += e.health;
    p.hunger += e.hunger;
    p.time += e.time;
}

fn main() {
    let skateboard: item = item::skateboard(); //Created a skateboard item
    let mut Cable = Player {
        // Player Object
        name: "Caleb".into(),
        hunger: 100,
        time: 100,
        health: 100,
        item: Some(skateboard),
    };

    // let mut testRoom: Room = Room::HarveyMudd(); // Created a room object
    // testRoom.display(); // This should print running in the console upon running, dont mind the 16 warnings LMAO

    let mut theGame: GameState = GameState {
        currentSchool: (School::HarveyMudd),
        schoolRoom: (0),
        intro: (true),
    };

    use std::io;
    use std::io::Write;

    let mut player: Player = Player::default();

    //First asking for the name of the player
    println!("What is your name"); //We can change the dialouges later lmao
    print!(">");
    let mut input = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    // let input = input.trim();
    player.name = input.trim().into();

    //Now getting the item the player wants
    println!("Hi {}, Choose a starting item", player.name);
    print!(">");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if input.to_lowercase() == "skateboard" {
        player.item = Some(item::skateboard());
    } else {
        player.item = Some(item::skateboard());
    }
    println!(
        "Your name is {} and you have a {}",
        player.name,
        player.item.unwrap().name
    );

    let schools = vec![Room::StartingRoom(), Room::Scripps()]; // This is going to be the vector that holds all the rooms
    let e = events::StarterEvent(); //Testing variables lol
    e.printEvents();

    let mut schoolCounter = 0;
    //This is the loop that will go through all the schools
    while schoolCounter < schools.len() {
        println!("-------------");
        println!("Player {}", player.name);
        println!(
            "Health: {}, Hunger: {}, Time: {}",
            player.health, player.hunger, player.time
        );
        println!("-------------");

        schoolCounter += 1;
    }
}
