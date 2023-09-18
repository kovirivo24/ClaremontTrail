

use std::ops::Deref;

// pub mod Room {
use crate::events;

    #[derive(Clone)]
    pub enum School {
        HarveyMudd,
        Pomona,
        Pitzer,
        Scripps,
        Cmc,
    }

    #[derive(Clone)]
    pub struct Room {
        name: String,
        message: String,
        school: School,
        pub megaEvent: Vec<events::events>,
        // roomEvents: Option<Vec<(events::events, i32)>>, // Max 1-3 
    }

    impl Room {
        //So what I was imaging was that when we create the rooms, the events are automatically created and attatched to those rooms
        pub fn StartingRoom() -> Room {
            Room {
                name: "String".into(),
                message: "String".into(),
                school: (School::HarveyMudd),
                megaEvent: vec![events::events::StarterEvent()],
            }
        }

        //The plan is to hopefully initialize a certain school with specific events that are generated when it's created.  i think we will have to tweek this a bit for the our frequency idea ??
        pub fn HarveyMudd() -> Room {
            Room {
                name: "Harvey Mudd".into(),
                message: "You start out in Harvey Mudd. Navigate to Pomona ASAP!".into(),
                school: (School::HarveyMudd),
                megaEvent: Vec::new(),
                // roomEvents: (Some(events::events::createevents())),
            }
        }

        pub fn Scripps() -> Room {
            Room {
                name: "String".into(),
                message: "String".into(),
                school: (School::Scripps),
                megaEvent: Vec::new(),
                // roomEvents: (Some(events::events::createevents())),
            }
        }

        pub fn CMC() -> Room {
            Room {
                name: "String".into(),
                message: "String".into(),
                school: (School::Cmc),
                megaEvent: Vec::new(),
                // roomEvents: (Some(events::events::createevents())),
            }
        }

        pub fn Pomona() -> Room {
            Room {
                name: "Pomona Campus".into(),
                message: "You have now entered Pomona, make your way to class quick!".into(),
                school: (School::Pomona),
                megaEvent: Vec::new(),
                // roomEvents: (Some(events::events::createevents())),
            }
        }

        pub fn add_megaevent(&mut self, mega_events: Vec<events::events>) {
            // if let Some(existing_events) = &mut self.events {
            //     existing_events.extend(new_events);
            // } else {
            //     self.events = Some(new_events);
            // }
            self.megaEvent.extend(mega_events);
        }

        pub fn get_message(&self) -> &str {
            &self.message
        }

        
        pub fn get_name(&self) -> &str {
            &self.name
        }
    }
// }
