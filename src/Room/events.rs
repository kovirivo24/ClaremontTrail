use std::clone::Clone;

pub mod events {

    #[derive(Clone)]
    pub struct events {
        pub name: String,
        pub message: String,
        pub megaEvent: bool,
        pub events: Option<Vec<events>>,
        pub chance: i8, // make this a float, or choose arbitary max
        pub time: i8,
        pub health: i8,
        pub hunger: i8,
    }

    // implements default values for each events -> let p = events { name: "Lunch", ..Default::default() };
    impl Default for events {
        fn default() -> events {
            events {
                name: "Untitledevents".into(),
                message: "EmptyMessage".into(),
                megaEvent: false,
                events: None,
                chance: 1, // lets set a default attributes
                time: -5,
                health: 0,
                hunger: 0,
            }
        }
    }

    // impl Clone for events {
    //     fn clone(&self) -> events {
    //         events {
    //             name: self.name.clone(),
    //             message: self.message.clone(),
    //             megaEvent: self.megaEvent.clone(),
    //             events: self.events.clone(),
    //             chance: self.chance.clone(),
    //             time: self.time.clone(),
    //             health: self.health.clone(),
    //             hunger: self.hunger.clone(),
    //         }
    //     }
    // }
    impl events {
        pub fn StarterEvent() -> events {
            events {
                name: "Starting Room".into(),
                message: "Your objective is to make it to your class".into(),
                megaEvent: false,
                events: Some(
                    (vec![
                        events {
                            name: "Skateboarding".into(),
                            message: "You skateboard.".into(),
                            megaEvent: false,
                            ..Default::default()
                        },
                        events {
                            name: "RunIntoStairs".into(),
                            message: "You just ran into a set of stairs. Ouch".into(),
                            megaEvent: false,
                            health: -2,
                            ..Default::default()
                        },
                    ]),
                ),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
            }
        }

        //Implementing Getters, I think it'll make lyfe easier down the line
        pub fn getName(self) -> String {
            self.name
        }

        pub fn getMessage(self) -> String {
            self.message
        }

        pub fn getMegaEvent(self) {}

        pub fn getEvents(self) {}

        pub fn addEvents(&mut self, mut event: Vec<events>) {
            // let mut curEvent = self.events;
            // if curEvent.is_none() {
            //     curEvent = Some(event)
            // } else {
            //     curEvent.unwrap().append(&mut event)
            // }
            // self.events = curEvent
        }

        pub fn getChance(self) -> i8 {
            self.chance
        }

        pub fn getHealth(self) -> i8 {
            self.health
        }
        pub fn getHunger(self) -> i8 {
            self.hunger
        }

        pub fn getTime(self) -> i8 {
            self.time
        }

        pub fn printEvents(self) {
            for i in self.events.iter().flatten() {
                println!("{}", i.name);
            }
        }
    }

    // going to have to restructure a lot of this
    // pub fn createevents() -> Vec<events> {
    //     // let events1 = {
    //     //     events {
    //     //         name: "Running".into(),
    //     //         message: "You run.".into(),
    //     //         megaEvent: false,
    //     //         ..Default::default()
    //     //     }
    //     // };
    //     // let events2 = {
    //     //     events {
    //     //         name: "LunchAtFrary".into(),
    //     //         message: "You are near Frary Dining Hall. Go in for lunch?".into(),
    //     //         megaEvent: true,
    //     //         events: Some(vec![events1]),
    //     //         chance: 1,
    //     //         time: 0,
    //     //         ..Default::default()
    //     //     }
    //     // };
    //     // let events = vec![events1, events2];

    //     //For some reason the above code had an issue with creating two variables then putting it into a vector ??, so i  just took one and put it inside the vector for testing purposses.
    //     let mut eventsVec = vec![
    //         events {
    //             name: "Running".into(),
    //             message: "You run.".into(),
    //             megaEvent: true,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "Skateboarding".into(),
    //             message: "You skateboard.".into(),
    //             megaEvent: false,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "RunIntoStairs".into(),
    //             message: "You just ran into a set of stairs. Ouch".into(),
    //             megaEvent: false,
    //             health: 2,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "RunIntoFootballPlayers".into(),
    //             message: "You just ran into football players. Ouch".into(),
    //             megaEvent: false,
    //             health: 5,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "RunIntoCar".into(),
    //             message: "You just ran into a car. Watch out".into(),
    //             megaEvent: false,
    //             health: 8,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "RunIntoCactus".into(),
    //             message: "You just ran into a cactus. Be careful".into(),
    //             megaEvent: false,
    //             health: 7,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "LunchAtFrary".into(),
    //             message: "You are near Frary Dining Hall. Go in for lunch?".into(),
    //             megaEvent: true,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "CrowdedLunch".into(),
    //             message: "The lunch is out the door. Literally. You lose time.".into(),
    //             chance: 7,
    //             time: 9,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "FoodPoisoning".into(),
    //             message: "The food you ate gave you food poisoning. You lose health and time."
    //                 .into(),
    //             chance: 1,
    //             time: 2,
    //             health: 5,
    //             ..Default::default()
    //         },
    //         events {
    //             name: "GreatLunch".into(),
    //             message: "The line was not too long. Your hunger is satisfied".into(),
    //             hunger: 8,
    //             ..Default::default()
    //         },
    //     ];

    //     let randfoodOptions = vec![eventsVec[2], eventsVec[3], eventsVec[4]];

    //     eventsVec[1].addEvents(randfoodOptions);

    //     // let eventsVec = vec![events {
    //     //     name: "Running".into(),
    //     //     message: "You run.".into(),
    //     //     megaEvent: false,
    //     //     ..Default::default()
    //     // }];

    //     let mut eventsVec = Vec::new();
    //     eventsVec.push(events {
    //         name: "Running".into(),
    //         message: "You run.".into(),
    //         megaEvent: false,
    //         ..Default::default()
    //     });
    //     return eventsVec;
    // }
}
