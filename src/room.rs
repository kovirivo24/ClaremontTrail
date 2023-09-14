

use std::ops::Deref;

// pub mod Room {
use crate::events;

    pub enum School {
        HarveyMudd,
        Pomona,
        Pitzer,
        Scripps,
        Cmc,
    }

    pub struct Room {
        school: School,
        megaEvent: Option<Vec<events::events>>,
        // roomEvents: Option<Vec<(events::events, i32)>>, // Max 1-3 
    }

    impl Room {
        pub fn StartingRoom() -> Room{
            Room {
                school: (School::HarveyMudd),
                megaEvent: (None),
                // roomEvents: (None), 
            }
        }

        //The plan is to hopefully initialize a certain school with specific events that are generated when it's created.  i think we will have to tweek this a bit for the our frequency idea ??
        pub fn HarveyMudd() -> Room {
            Room {
                school: (School::HarveyMudd),
                megaEvent: (None),
                // roomEvents: (Some(events::createevents())),
            }
        }

        pub fn addMegaEvent(&mut self, mut event: Vec<events::events>) {
            let mut curEvent = self.megaEvent.clone();
            if curEvent.is_none() {
                curEvent = Some(event.to_vec());
                self.megaEvent = curEvent
            } else {
                let mut eventlist = curEvent.unwrap(); // just dont worry about trhis its 1 am and im trying to make it work
                eventlist.append(&mut event);
                self.megaEvent = Some(eventlist)
            }
        }
        //This function is just for testing and displaying an event name that the room has
        pub fn display(self) {
            println!("TEST NAME{}", self.megaEvent.unwrap()[1].clone().getName());
        }
    }
// }
