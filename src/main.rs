#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

mod events;
mod player; // iimporting the player module
mod room; // importing the room module
mod item;
use std::ops::Deref;

// use player::item::item::item; // since player imports item, we don't have to re-import it but jsut "use" it
use player::Player;  //Mod player and struct player is different, so we are using Player structure
use room::Room;

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
fn statAdjuster(p: &mut Player, e: &events::events) {
    p.health += e.getHealth();
    p.hunger += e.getHunger();
    p.time += e.getTime();
}

fn printEvents(mega: &Vec<events::events>) {
    let mut counter = 0;
    for i in mega {
        println!("({}): {}", counter, i.getName());
        counter += 1;
    }
}

// fn eventSelector(es: &Vec<events::events {
    
// }>, choice: u8) {}

fn main() {
    let eventList = events::createevents();
    let skateboard = item::skateboard() ;//Created a skateboard item
    let mut Cable = Player { // Player Object
        name: "Caleb".into(),
        hunger: 100,
        time: 100,
        health: 100,
        item: Some(skateboard),
    };

    let mut testRoom: Room = Room::HarveyMudd(); // Created a room object
    let mut mudd_megaevents = vec![eventList[0].0.clone(), eventList[6].0.clone()];
    // testRoom.addMegaEvent(mudd_megaevents);
    // testRoom.display(); // This should print running in the console upon running, dont mind the 16 warnings LMAO

    for mega in mudd_megaevents.iter_mut() {
        if mega.getEvents().is_none(){
            let mut cur_message = mega.getMessage();
            let mut options = mega.getEvents().clone().unwrap();
            let mut opt_num = 0; 
            for option in options.iter_mut(){
                opt_num += 1;
                println!("{}: {}", opt_num, option.getMessage())
            }
            println!("{}{}", cur_message, mega.getName() )
        }
    }


    let mut theGame: GameState = GameState { currentSchool: (School::HarveyMudd), schoolRoom: (0), intro: (true) };
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
    let e = events::events::StarterEvent(); //Testing variables lol
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



   