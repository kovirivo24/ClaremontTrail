#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

// mod events;
mod player;
use player::item::item::item;
use player::player::Player;

// pub mod item;

enum School {
    HarveyMudd,
    Pomona,
    Pitzer,
    Scripps,
    Cmc,
}

// struct Room {
//     school: School,
//     megaEvent: Option<events::Event>,
//     events: Option<Vec<events::Event>>, // Max 1-3
// }

fn main() {
    let skateboard: item = item::skateboard();
    let mut Cable = Player {
        name: "Caleb".into(),
        hunger: 100,
        time: 100,
        health: 100,
        item: skateboard,
    };

    // let testRoom: Room = Room {
    //     school: School::HarveyMudd,
    //     megaEvent: None,
    //     events: None,
    // };
}
