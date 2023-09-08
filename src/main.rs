mod events;

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
    time_mult: f32, // for skateboard
    hunger_effect: u8, // e.g. snack
    health_effect: u8, // e.g. someone poisoined ur snack :(
}

impl Default for Item {
    fn default() -> Item {
        Item {
            name: "UntitledItem".into(),
            time_mult: 1.0, // for skateboard
            hunger_effect: 0, // e.g. snack
            health_effect: 0, // e.g. someone poisoined ur snack :(
        }
    }
}

struct Room {
    school: School,
    megaEvent: Option<events::Event>,
    events: Option<Vec<events::Event>> // Max 1-3
}

fn main() {
    let mut Cable = Player{name: "Caleb".into(), hunger: 100, time: 100, health:100, item: Item { ..Default::default()} };
    let testRoom: Room = Room{school: School::HarveyMudd, megaEvent: None, events: None};
}

