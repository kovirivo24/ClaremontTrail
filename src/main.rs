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

fn print_mega_options(mega_event: events::events) {
    if !mega_event.get_events().is_empty(){
        let mut options = mega_event.get_events().clone();
        let mut counter = 0; 
        for option in options.iter_mut(){
            counter += 1;
            println!("({}): {}", counter, option.getMessage());
        }
    }
}
fn printEvents(mega: &Vec<events::events>) {
    let mut counter = 0;
    for i in mega {
        println!("({}): {}", counter, i.getName());
        counter += 1;
    }
}

fn print_user_info(player: Player) {
    println!("-------------");
    println!("Player: {}", player.name);
    println!(
        "Health: {}, Hunger: {}, Time: {}",
        player.health, player.hunger, player.time
    );
    println!("-------------");

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

    // let mut testRoom: Room = Room::HarveyMudd(); // Created a room object
    let mut mudd_megaevents = vec![eventList[0].clone(), eventList[6].clone()];
    // testRoom.addMegaEvent(mudd_megaevents);
    // testRoom.display(); // This should print running in the console upon running, dont mind the 16 warnings LMAO

    for mega in mudd_megaevents.iter_mut() {
        let cur_message = mega.getMessage();
        println!("message of mega event: {} name of event: {}", cur_message, mega.getName() );
        if !mega.get_events().is_empty(){
            let mut options = mega.get_events().clone();
            let mut opt_num = 0; 
            for option in options.iter_mut(){
                opt_num += 1;
                println!("{}: Mega Option {}", opt_num, option.getMessage());
                let option_index = events::find_event_index(&eventList, option.getName());
                println!("optionindex {}", option_index);
                let option_events = eventList.get(option_index as usize).clone().unwrap().get_events();
                for option_event in option_events {
                    println!("Option event {}", option_event.getMessage());
                }

            }
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

    if input.to_lowercase()== "skateboard" {
        player.item = Some(item::skateboard());
    } else {
        player.item = Some(item::skateboard());
    }
    println!(
        "Your name is {} and you have a {}",
        player.name,
        player.item.unwrap().name
    );

    let mut mudd_room = Room::HarveyMudd();
    let mut pomona_room = Room::Pomona();

    // run three times
    let mudd_mega = vec![eventList[0].clone(), eventList[events::find_event_index(&eventList, "TSwift") as usize].clone(),eventList[0].clone(), eventList[0].clone()];
    mudd_room.add_megaevent(mudd_mega);

    let schools = vec![mudd_room, pomona_room]; // This is going to be the vector that holds all the rooms

    let mut schoolCounter = 0;
    //This is the loop that will go through all the schools
    // while schoolCounter < schools.len() {
    for school in schools {
        // let cloneVec = schools.clone();
        // print_user_info(player.clone());


        let mega_event_list = school.megaEvent;
        for mega_event in mega_event_list {
            println!("{}", mega_event.getMessage());
            print_mega_options(mega_event);

            println!("Choose an option");
        print!(">");

        //Storing their choice
        let mut choice = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        let number: usize = choice.parse().unwrap();
        }


        // printEvents(&cloneVec[schoolCounter].megaEvent.unwrap()[0].events.clone().unwrap());
        /* 
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

        // schoolCounter += 1;

        */
    } 
}



   