


pub struct Event {
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
            name: "UntitledEvent".into(),
            message: "EmptyMessage".into(),
            megaevent: False,
            events: None, //set to walk?
            chance: 1, // lets set a default attributes
            time: 5,
            health: 0,
            hunger: 0,
        }
    }
}

let events = [
    Event {
        name: "Running",
        message: "You run.",
        megaEvent: False,
        ..Default::default()
    },
    Event {
        name: "LunchAtFrary",
        message: "You are near Frary Dining Hall. Go in for lunch?",
        megaEvent: True,
        events: vec![events[0]],
        chance: 1,
        time: 0,
        ..Default::default()
    }

]