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

fn main() {
    let eventList = events::createevents();
    let skateboard = item::skateboard() ;//Created a skateboard item
    let mut Cable = Player { // Player Object
        name: "Caleb".into(),
        hunger: 100,
        time: 100,
        health: 100,
        item: skateboard,
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
    use std::io;
    use std::io::Write;



}



   