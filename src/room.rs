

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
        school: School,
        pub megaEvent: Option<Vec<events::events>>,
        // roomEvents: Option<Vec<(events::events, i32)>>, // Max 1-3 
    }

    impl Room {
        //So what I was imaging was that when we create the rooms, the events are automatically created and attatched to those rooms
        pub fn StartingRoom() -> Room {
            Room {
                school: (School::HarveyMudd),
                megaEvent: Some(vec![events::events::StarterEvent()]),
            }
        }

        // pub fn HarveyMudd() -> Room {
        //     Room {
        //         school: (School::HarveyMudd),
        //         megaEvent: (None),
        //        //  // roomEvents: (Some(events::events::createevents())), 
        //     }
        // }

        //The plan is to hopefully initialize a certain school with specific events that are generated when it's created.  i think we will have to tweek this a bit for the our frequency idea ??
        pub fn HarveyMudd() -> Room {
            Room {
                school: (School::HarveyMudd),
                megaEvent: (None),
                // roomEvents: (Some(events::events::createevents())),
            }
        }
        //This function is just for testing and displaying an event name that the room has
        pub fn display(self) {
            // println!("{}", self.roomEvents.unwrap()[0].getName());
        }
    }
// }
