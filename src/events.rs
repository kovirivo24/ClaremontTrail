
pub struct Event {
    name: String,
    message: String,
    mega_event: bool, 
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
            mega_event: false,
            events: None, //set to walk?
            chance: 1, // lets set a default attributes
            time: 5,
            health: 0,
            hunger: 0,
        }
    }
}

// going to have to restructure a lot of this
fn createEvents() -> [Event] {
    let event1 = {
        Event {
            name: "Running".into(),
            message: "You run.".into(),
            mega_event: false,
            ..Default::default()
        }
    };
    let event2 =  {
        Event {
            name: "LunchAtFrary".into(),
            message: "You are near Frary Dining Hall. Go in for lunch?".into(),
            mega_event: true,
            events: Some(vec![event1]),
            chance: 1,
            time: 0,
            ..Default::default()
        }
    };
    let events = [
        event1,
        event2
    ];
    return events
} 
