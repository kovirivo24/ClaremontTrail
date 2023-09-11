pub mod events {
    pub struct events {
        pub name: String,
        message: String,
        megaEvent: bool,
        events: Option<Vec<events>>,
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
                megaEvent: false,
                events: None,
                chance: 1,    // lets set a default attributes
                time: 5,
                health: 0,
                hunger: 0,
            }
        }
    }
    impl events {
        //Implementing Getters, I think it'll make lyfe easier down the line
        pub fn getName(self) -> String {
            (self.name)
        }
    
        pub fn getMessage(self) -> String {
            (self.message)
        }
    
        pub fn getMegaEvent(self) {}
    
        pub fn getEvents(self) {}
    
        pub fn addEvents(&mut self, mut event: Vec<events>) {
            let mut curEvent = self.events;
            if curEvent.is_none() {
                curEvent = Some(event)
            } else {
                curEvent.unwrap().append(&mut event)
            }
            self.events = curEvent
        }
    
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
        let mut eventsVec = vec![
            events {
                name: "Running".into(),
                message: "You run.".into(),
                megaEvent: true,
                ..Default::default()
            },
            events {
                name: "Skateboarding".into(),
                message: "You skateboard.".into(),
                megaEvent: true,
                ..Default::default()
            },
            events {
                name: "RunIntoStairs".into(),
                message: "You just ran into a set of stairs. Ouch".into(),
                megaEvent: false,
                health: 2,
                ..Default::default()
            },
            events {
                name: "RunIntoFootballPlayers".into(),
                message: "You just ran into football players. Ouch".into(),
                megaEvent: false,
                health: 5,
                ..Default::default()
            },
            events {
                name: "RunIntoCar".into(),
                message: "You just ran into a car. Watch out".into(),
                megaEvent: false,
                health: 8,
                ..Default::default()
            },
            events {
                name: "RunIntoCactus".into(),
                message: "You just ran into a cactus. Be careful".into(),
                megaEvent: false,
                health: 7,
                ..Default::default()
            },
            events {
                name: "LunchAtFrary".into(),
                message: "You are near Frary Dining Hall. Go in for lunch?".into(),
                megaEvent: true,
                ..Default::default()
            },
            events {
                name: "CrowdedLunch".into(),
                message: "The lunch is out the door. Literally. You lose time.".into(),
                chance: 7,
                time: 9,
                ..Default::default()
            },
            events {
                name: "FoodPoisoning".into(),
                message: "The food you ate gave you food poisoning. You lose health and time.".into(),
                chance: 1,
                time: 2,
                health: 5,
                ..Default::default()
            },
            events {
                name: "GreatLunch".into(),
                message: "The line was not too long. Your hunger is satisfied".into(),
                hunger: 8,
                ..Default::default()
            },
        ];
        
        let randfoodOptions = vec![eventsVec[2], eventsVec[3], eventsVec[4]];
    
        eventsVec[1].addEvents(randfoodOptions);
        
    
        return eventsVec;
    }
    
}
