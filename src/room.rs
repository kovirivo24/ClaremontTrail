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
        pub mega_event: Vec<events::Event>,
        // roomEvents: Option<Vec<(events::events, i32)>>, // Max 1-3 
    }

    impl Room {
        //So what I was imaging was that when we create the rooms, the events are automatically created and attatched to those rooms
        pub fn starting_room() -> Room {
            Room {
                name: "String".into(),
                message: "String".into(),
                school: (School::HarveyMudd),
                mega_event: vec![events::Event::starter_event()],
            }
        }

        //The plan is to hopefully initialize a certain school with specific events that are generated when it's created.  i think we will have to tweek this a bit for the our frequency idea ??
        pub fn harvey_mudd() -> Room {
            Room {
                name: "Harvey Mudd".into(),
                message: "You start out in Harvey Mudd. Navigate to Pomona ASAP!".into(),
                school: (School::HarveyMudd),
                mega_event: Vec::new(),
                // roomEvents: (Some(events::events::createevents())),
            }
        }

        pub fn pomona() -> Room {
            Room {
                name: "Pomona Campus".into(),
                message: "You have now entered Pomona, make your way to class quick!".into(),
                school: (School::Pomona),
                mega_event: Vec::new(),
                // roomEvents: (Some(events::events::createevents())),
            }
        }

        pub fn add_mega_event(&mut self, mega_events: Vec<events::Event>) {
            // if let Some(existing_events) = &mut self.events {
            //     existing_events.extend(new_events);
            // } else {
            //     self.events = Some(new_events);
            // }
            self.mega_event.extend(mega_events);
        }

        pub fn get_message(&self) -> &str {
            &self.message
        }

        
        pub fn get_name(&self) -> &str {
            &self.name
        }
    }
// }
