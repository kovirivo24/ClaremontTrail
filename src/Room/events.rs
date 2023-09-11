use std::clone::Clone;

pub mod events {

    #[derive(Clone)]
    pub struct events {
        name: String,
        message: String,
        mega_events: bool,
        events: Option<Vec<events>>, // eventss can lead to other eventss,
        chance: u8,                  // make this a float, or choose arbitary max
        time: u8,
        health: u8,
        hunger: u8,
    }

    // implements default values for each events -> let p = events { name: "Lunch", ..Default::default() };
    impl Default for events {
        fn default() -> events {
            events {
                name: "Untitledevents".into(),
                message: "EmptyMessage".into(),
                mega_events: false,
                events: None, //set to walk? -- wdym by set to walk?
                chance: 1,    // lets set a default attributes
                time: 5,
                health: 0,
                hunger: 0,
            }
        }
    }

    impl Clone for events {
        fn clone(&self) -> events {
            events {
                name: self.name.clone(),
                message: self.message.clone(),
                mega_events: self.mega_events.clone(),
                events: self.events.clone(),
                chance: self.chance.clone(),
                time: self.time.clone(),
                health: self.health.clone(),
                hunger: self.hunger.clone(),
            };
        }
    }
    impl events {
        pub fn StarterEvent() -> events {
            events {
                name: "Starting Room".into(),
                message: "Your objective is to make it to your class".into(),
                mega_events: false,
                events: (None),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
            }
        }

        //Implementing Getters, I think it'll make lyfe easier down the line
        pub fn getName(self) -> String {
            (self.name)
        }

        pub fn getMessage(self) -> String {
            (self.message)
        }

        pub fn getMegaEvent(self) {}

        pub fn getEvents(self) {}

        pub fn getChance(self) -> u8 {
            (self.chance)
        }

        pub fn getHealth(self) -> u8 {
            (self.health)
        }
        pub fn getHunger(self) -> u8 {
            (self.hunger)
        }
    }

    // going to have to restructure a lot of this
    pub fn createevents() -> Vec<events> {
        // let events1 = {
        //     events {
        //         name: "Running".into(),
        //         message: "You run.".into(),
        //         mega_events: false,
        //         ..Default::default()
        //     }
        // };
        // let events2 = {
        //     events {
        //         name: "LunchAtFrary".into(),
        //         message: "You are near Frary Dining Hall. Go in for lunch?".into(),
        //         mega_events: true,
        //         events: Some(vec![events1]),
        //         chance: 1,
        //         time: 0,
        //         ..Default::default()
        //     }
        // };
        // let events = vec![events1, events2];

        //For some reason the above code had an issue with creating two variables then putting it into a vector ??, so i  just took one and put it inside the vector for testing purposses.
        // let eventsVec = vec![events {
        //     name: "Running".into(),
        //     message: "You run.".into(),
        //     mega_events: false,
        //     ..Default::default()
        // }];

        let mut eventsVec = Vec::new();
        eventsVec.push(events {
            name: "Running".into(),
            message: "You run.".into(),
            mega_events: false,
            ..Default::default()
        });
        return eventsVec;
    }
}
