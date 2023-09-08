enum School {
    HarveyMudd,
    Pomona,
    Pitzer,
    Scripps,
    Cmc
}

struct Player {
    name: String,
    hunger: u8, // Biggest Number in a u8 is 255
    time: u8, // Maybe make this a float , look at later -agreed Kovit
    health: u8,
    item: Item,
}

impl Player {
    pub fn test(&mut self){
        println!("{}",self.hunger)
    }
}

struct Item {
    name: String,
    timeMult: f32, // for skateboard
    hungerEffect: u8, // e.g. snack
    healthEffect: u8, // e.g. someone poisoined ur snack :(
}

impl Default for Item {
    fn default() -> Item {
        Item {
            name: "UntitledItem",
            timeMult: 1, // for skateboard
            hungerEffect: 0, // e.g. snack
            healthEffect: 0, // e.g. someone poisoined ur snack :(
        }
    }
}

struct Room {
    school: School,
    megaEvent: Option<Event>,
    events: Option<Vec<Event>> // Max 1-3
}


fn main() {
    let mut Cable = Player{name: "Caleb".into(), hunger: 100, time: 100, health:100, item: "Skateboard".into()};
    let testRoom: Room = Room{school: School::HarveyMudd, megaEvent: None, events: None};
}

