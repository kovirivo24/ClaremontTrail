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
// fn statAdjuster(p: &mut Player, e: &events::events) {
//     p.health += e.getHealth();
//     p.hunger += e.getHunger();
//     p.time += e.getTime();
// }

fn player_death(player: &Player) -> bool {
    player.health <= 0
}

fn player_out_time(player: &Player) -> bool {
    player.time <= 0
}

fn player_hungry(player: &Player) -> bool {
    player.hunger <= 0
}

fn print_mega_options(mega_event: &events::events) {
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

fn print_user_info(player: &Player, room: &Room) {
    println!("-------------");
    println!("Player: {}", player.name);
    println!("Location: {}", room.get_name());
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

    println!("Hi {}. You are about to leave class", player.name);
    print!("Press enter to continue");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    println!(
        "Your name is {} and you have a skateboard. Feel free to use it.",
        player.name,
    );

    let mut mudd_room = Room::HarveyMudd();
    let mut pomona_room = Room::Pomona();

    // run three times
    let mudd_mega = vec![eventList[0].clone(), eventList[events::find_event_index(&eventList, "TSwift") as usize].clone(),eventList[0].clone(), eventList[0].clone()];
    mudd_room.add_megaevent(mudd_mega);

    let pomona_mega = vec![eventList[0].clone(),eventList[0].clone(), eventList[events::find_event_index(&eventList, "LunchAtFrary") as usize].clone(),eventList[0].clone(), eventList[0].clone()];
    pomona_room.add_megaevent(pomona_mega);

    let schools = vec![mudd_room, pomona_room]; // This is going to be the vector that holds all the rooms

    //This is the loop that will go through all the schools
    // while schoolCounter < schools.len() {
    for school in schools {
        // let cloneVec = schools.clone();
        // print_user_info(&player, &school);
        println!("{}", school.get_message());
        print!("Press enter to continue");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let mega_event_list = school.megaEvent.clone();
        for mega_event in mega_event_list {
            print_user_info(&player, &school);
            println!("{}", mega_event.getMessage());
            print_mega_options(&mega_event);
    
            println!("Choose an option");
            print!(">");

            //Storing their choice
            let mut choice = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut choice).unwrap();
            let choice = choice.trim();

            while choice.parse::<usize>().unwrap() > mega_event.get_events().len() || choice.parse::<usize>().unwrap() < 1 {
                println!("Choose an option");
                print!(">");

                let mut choice = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut choice).unwrap();
            }

            let number: usize = choice.parse::<usize>().unwrap() - 1;

            let event_choice_name = mega_event.get_events().get(number).unwrap().getName();
            let event_choice_consq = eventList.get(events::find_event_index(&eventList, event_choice_name) as usize).unwrap().get_events();
            let event_choice_consq_index = eventList.get(events::find_event_index(&eventList, event_choice_name) as usize).unwrap().randEvent();
            let event_consq = event_choice_consq.get(event_choice_consq_index as usize).unwrap().clone();
            
            let (consq_health, consq_hunger, consq_time) = (event_consq.getHealth(),  event_consq.getHunger(), event_consq.getTime());
            player.updateHealth(consq_health);
            player.updateHunger(consq_hunger);
            player.updateTime(consq_time);

            if player_death(&player) {
                println!(
                    "You could not survive the Claremont Trail. Oof, that's pathetic!",
                )
            } else if player_out_time(&player) {
                println!(
                    "You ran out of time. Sucks to suck",
                )
            } 
            if player_hungry(&player) {
                player.updateHealth(-10);
                println!(
                    "Your are very hungry. Eat something soon!",
                );
            }

            println!("{}", event_consq.getMessage());

            println!(
                "Your health was affected by {}, your hunger was affected by {}, your time was affected by {}",
                consq_health, consq_hunger, consq_time
            );
            
            print!("Press enter to continue");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
        }
    } 

    print!("Congrats! You made it to class! To play again, press enter!");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if input == "\n" {
        main();
    } 
}




   