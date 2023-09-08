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


struct Event {
    name: String,
    message: String,
    megaevent: Bool, 
    events: Option<Vec<Event>>, // events can lead to other events, 
    chance: u8, // make this a float, or choose arbitary max
    time: u8,
    health: u8,
    hunger: u8, 
}

// implements default values for each event -> let p = Event { name: "Lunch", ..Default::default() };
impl Default for Event {
    fn default() -> Event {
        Event {
            name: "UntitledEvent",
            message: "EmptyMessage",
            megaevent: False,
            events: None, //set to walk?
            chance: 1, // lets set a default attributes
            time: 5,
            health: 0,
            hunger: 0,
        }
    }
}



fn main() {
    let mut Cable = Player{name: "Caleb".into(), hunger: 100, time: 100, health:100, item: "Skateboard".into()};
    let testRoom: Room = Room{school: School::HarveyMudd, megaEvent: None, events: None};
}

