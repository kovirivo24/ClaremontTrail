#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

mod events;
mod player; // iimporting the player module
mod room; // importing the room module
mod item;

// use player::Item::Item::Item; // since player imports Item, we don't have to re-import it but jsut "use" it
use player::Player;  //Mod player and struct player is different, so we are using Player structure
use room::Room;

pub enum School {
    HarveyMudd,
    Pomona,
    Pitzer,
    Scripps,
    Cmc,
}

fn player_death(player: &Player) -> bool {
    player.health <= 0
}

fn player_out_time(player: &Player) -> bool {
    player.time <= 0
}

fn player_hungry(player: &Player) -> bool {
    player.hunger <= 0
}

fn print_mega_options(mega_event: &events::Event) {
    if !mega_event.get_events().is_empty(){
        let mut options = mega_event.get_events().clone();
        let mut counter = 0; 
        for option in options.iter_mut(){
            counter += 1;
            println!("({}): {}", counter, option.get_message());
        }
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
    let event_list = events::createevents();
    use std::io;
    use std::io::Write;

    let mut player: Player = Player::default();

    //First asking for the name of the player
    println!("What is your name?"); //We can change the dialouges later lmao
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

    let mut mudd_room = Room::harvey_mudd();
    let mut pomona_room = Room::pomona();

    // run three times
    let mudd_mega = vec![event_list[0].clone(), event_list[events::find_event_index(&event_list, "TSwift") as usize].clone(),event_list[0].clone(), event_list[0].clone()];
    mudd_room.add_mega_event(mudd_mega);

    let pomona_mega = vec![event_list[0].clone(),event_list[0].clone(), event_list[events::find_event_index(&event_list, "LunchAtFrary") as usize].clone(),event_list[0].clone(), event_list[0].clone()];
    pomona_room.add_mega_event(pomona_mega);

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

        let mega_event_list = school.mega_event.clone();
        for mega_event in mega_event_list {
            print_user_info(&player, &school);
            println!("{}", mega_event.get_message());
            print_mega_options(&mega_event);
    
            println!("Choose an option");
            print!(">");

            //Storing their choice
            let mut choice = String::new();
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut choice).unwrap();

            // wrong input logic not working
            while choice.is_empty() || choice.trim().parse::<usize>().unwrap() > mega_event.get_events().len() || choice.trim().parse::<usize>().unwrap() < 1 {
                println!("Choose an option");
                print!(">");
                let mut choice = String::new();
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut choice).unwrap();
            }
            let choice = choice.trim();

            let number: usize = choice.parse::<usize>().unwrap() - 1;

            let event_choice_name = mega_event.get_events().get(number).unwrap().get_name();
            let event_choice_consq = event_list.get(events::find_event_index(&event_list, event_choice_name) as usize).unwrap().get_events();
            let event_choice_consq_index = event_list.get(events::find_event_index(&event_list, event_choice_name) as usize).unwrap().rand_event();
            let event_consq = event_choice_consq.get(event_choice_consq_index as usize).unwrap().clone();
            
            let (mut consq_health, mut consq_hunger, mut consq_time) = (event_consq.get_health(),  event_consq.get_hunger(), event_consq.get_time());
            if event_choice_name == "ContinueToSkateboard" {
                consq_health += 5;
                consq_hunger += 3;
                consq_time -= 4;

            }
            player.update_health(consq_health);
            player.update_hunger(consq_hunger);
            player.update_time(consq_time);

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
                player.update_health(-10);
                println!(
                    "Your are very hungry. Eat something soon!",
                );
            }

            println!("{}", event_consq.get_message());

            println!(
                "Your health was affected by {}, your hunger was affected by {}, your time was affected by {}",
                consq_health, consq_hunger, consq_time
            );
            
            print!("Press enter to continue");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
        }
    } 

    println!("Congrats! You made it to class! To play again, press enter!");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if input.is_empty() {
        main();
    } 
}




   