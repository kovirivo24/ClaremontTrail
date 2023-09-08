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
    time: u8, // Maybe make this a float , look at later
    health: u8,
    item: String,
}

impl Player {
    pub fn test(&mut self){
        println!("{}",self.hunger)
    }
}


struct Room {
    school: School,
    megaEvent: Option<Event>,
    events: Option<Vec<Event>> // Max 1-3
}


struct Event {
    name: String,
    description: string,
    chance: u8, // make this a float, or choose arbitary max
    time: u8,
    health: u8,
    hunger: u8, 
}



fn main() {
    let mut Cable = Player{name: "Caleb".into(), hunger: 100, time: 100, health:100, item: "Skateboard".into()};
    let testRoom: Room = Room{school: School::HarveyMudd, megaEvent: None, events: None};
}

