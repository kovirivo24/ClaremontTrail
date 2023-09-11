mod events;

pub mod Room {
    use super::events;

    pub enum School {
        HarveyMudd,
        Pomona,
        Pitzer,
        Scripps,
        Cmc,
    }

    pub struct Room {
        school: School,
        megaEvent: Option<events::events::events>,
        roomEvents: Option<Vec<events::events::events>>, // Max 1-3
    }

    impl Room {
        pub fn StartingRoom() -> Room{
            Room {
                school: (School::HarveyMudd),
                megaEvent: (None),
                roomEvents: (None),
            }
        }

        //The plan is to hopefully initialize a certain school with specific events that are generated when it's created.  i think we will have to tweek this a bit for the our frequency idea ??
        pub fn HarveyMudd() -> Room {
            Room {
                school: (School::HarveyMudd),
                megaEvent: (None),
                roomEvents: (Some(events::events::createevents())),
            }
        }
        //This function is just for testing and displaying an event name that the room has
        pub fn display(self) {
            println!("{}", self.roomEvents.unwrap()[0].getName());
        }
    }
}
