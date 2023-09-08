#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

// mod events;
mod player; // iimporting the player module
mod room; // importing the room module
use player::item::item::item; // since player imports item, we don't have to re-import it but jsut "use" it
use player::player::Player;  //Mod player and struct player is different, so we are using Player structure
use room::Room::Room; // Same thing but for the room mod

// pub mod item;
pub enum School {
    HarveyMudd,
    Pomona,
    Pitzer,
    Scripps,
    Cmc,
}

fn main() {
    let skateboard: item = item::skateboard(); //Created a skateboard item
    let mut Cable = Player { // Player Object
        name: "Caleb".into(),
        hunger: 100,
        time: 100,
        health: 100,
        item: skateboard,
    };

    let mut testRoom: Room = Room::HarveyMudd(); // Created a room object
    testRoom.display(); // This should print running in the console upon running, dont mind the 16 warnings LMAO
}
