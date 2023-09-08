#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

// mod events;
mod player;
mod room;
use player::item::item::item;
use player::player::Player;

// pub mod item;
pub enum School {
    HarveyMudd,
    Pomona,
    Pitzer,
    Scripps,
    Cmc,
}




fn main() {
    let skateboard: item = item::skateboard();
    let mut Cable = Player {
        name: "Caleb".into(),
        hunger: 100,
        time: 100,
        health: 100,
        item: skateboard,
    };

    //PLan when we create a room, the events and such are automatically created.
    // let testRoom: room = room {school: School::HarveyMudd, mega_events:None, events:None};
    // let testRoom: Room = Room {
    //     school: School::HarveyMudd,
    //     megaEvent: None,
    //     events: None,
    // };
}
