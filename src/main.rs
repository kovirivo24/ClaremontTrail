#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

// mod events;
mod player; // iimporting the player module
mod room;
use std::clone;

// importing the room module
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

fn printEvents(mega: &Vec<events>) {
    let mut counter = 0;
    for i in mega {
        println!("({}): {}", counter, i.name);
        counter += 1;
    }
}

fn eventSelector(es: &Vec<events>, choice: u8) {}

fn main() {
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

    let schools = vec![Room::StartingRoom(), Room::StartingRoom()]; // This is going to be the vector that holds all the rooms
    let e = events::StarterEvent(); //Testing variables lol
                                    // printEvents(&e.events.unwrap());

    let mut schoolCounter = 0;
    //This is the loop that will go through all the schools
    while schoolCounter < schools.len() {
        let cloneVec = schools.clone();
        println!("-------------");
        println!("Player: {}", player.name);
        println!(
            "Health: {}, Hunger: {}, Time: {}",
            player.health, player.hunger, player.time
        );
        println!("-------------");

        printEvents(&cloneVec[schoolCounter].megaEvent.events.clone().unwrap());

        println!("Choose an option");
        print!(">");

        //Storing their choice
        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        let number: usize = choice.parse().unwrap();

        println!(
            "{} Your health was affected by {}, your hunger was affected by {}, your time was affected by {}",
            &cloneVec[schoolCounter].megaEvent.events.clone().unwrap()[number].message, 
            &cloneVec[schoolCounter].megaEvent.events.clone().unwrap()[number].health,
            &cloneVec[schoolCounter].megaEvent.events.clone().unwrap()[number].hunger,
            &cloneVec[schoolCounter].megaEvent.events.clone().unwrap()[number].time,
        );

        player.health += &cloneVec[schoolCounter].megaEvent.events.clone().unwrap()[number].health;
        player.hunger += &cloneVec[schoolCounter].megaEvent.events.clone().unwrap()[number].hunger;
        player.time += &cloneVec[schoolCounter].megaEvent.events.clone().unwrap()[number].time;

        schoolCounter += 1;
    }
}
