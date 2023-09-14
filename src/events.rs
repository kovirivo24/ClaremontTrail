// pub mod events {
    use weighted_rand::builder::*;
    use std::{clone::Clone, ops::Deref};


    #[derive(Clone)]
    pub struct events {
        name: String,
        message: String,
        megaEvent: bool,
        events: Option<Vec<events>>,
        chance: u8,                  // make this a float, or choose arbitary max
        time: i8,
        health: i8,
        hunger: i8,
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

    // impl Clone for events {
    //     fn clone(&self) -> events {
    //         return events {
    //             name: self.name.clone(),
    //             message: self.message.clone(),
    //             megaEvent: self.megaEvent.clone(),
    //             events: self.events.clone(),
    //             chance: self.chance.clone(),
    //             time: self.time.clone(),
    //             health: self.health.clone(),
    //             hunger: self.hunger.clone(),
    //         };
    //     }
    // }
    impl events {
        pub fn StarterEvent() -> events {
            events {
                name: "Starting Room".into(),
                message: "Your objective is to make it to your class".into(),
                megaEvent: false,
                events: (None),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
            }
        }

        //Implementing Getters, I think it'll make lyfe easier down the line
        pub fn getName(&self) -> &str {
            &self.name
        }

        pub fn getMessage(&self) -> &str {
           &self.message
        }

        pub fn getMegaEvent(&self) {}

        pub fn getEvents(&self) -> &Option<Vec<events>>{
            &self.events
        }

        pub fn addEvents(&mut self, mut event: Vec<events>) {
            let mut curEvent = self.events.clone();
            if curEvent.is_none() {
                curEvent = Some(event.to_vec());
                self.events = curEvent
            } else {
                let mut eventlist = curEvent.unwrap(); // just dont worry about trhis its 1 am and im trying to make it work
                eventlist.append(&mut event);
                self.events = Some(eventlist)
            }
        }

        pub fn getChance(&self) -> u8 {
            (self.chance)
        }

        pub fn getHealth(&self) -> i8 {
            (self.health)
        }
        pub fn getHunger(&self) -> i8 {
            (self.hunger)
        }

        pub fn randEvent(&self) -> i32 {
            if self.events.is_some() {

                let choices = self.events.clone().unwrap();
                let mut weights = Vec::new();
                let mut total_weight = 0;
                for cur_event in choices {
                    weights.push(cur_event.chance as u32);
                    total_weight += cur_event.chance as u32;
                }
                let builder = WalkerTableBuilder::new(&weights);
                let wa_table = builder.build();

                for i in (0..total_weight).map(|_| wa_table.next()) {
                    total_weight = i as u32;
                }

                return total_weight as i32
                
            } else {
                return -1
            }
        }
    }

    fn swap_vec(vec: &mut Vec<(events, i32)>, eventID: i32, variant: events) {
        if let Some(item) = vec.iter_mut().find(|item| item.1 == eventID) {
            let mut temp = (events::StarterEvent(), eventID); // create placeholder value
            std::mem::swap(item, &mut temp); // swap the placeholder in
            *item = (variant, eventID); // overwrite with real value
        }
    }
    // going to have to restructure a lot of this
    pub fn createevents() -> Vec<(events, i32)> {
        //For some reason the above code had an issue with creating two variables then putting it into a vector ??, so i  just took one and put it inside the vector for testing purposses.
        let mut eventsVec = vec![
            (events {
                name: "Running".into(),
                message: "You run.".into(),
                megaEvent: true,
                ..Default::default()
            }, 0),
            (events {
                name: "Skateboarding".into(),
                message: "You skateboard.".into(),
                megaEvent: true,
                ..Default::default()
            }, 1),
            (events {
                name: "RunIntoStairs".into(),
                message: "You just ran into a set of stairs. Ouch".into(),
                megaEvent: false,
                health: 2,
                ..Default::default()
            }, 2),
            (events {
                name: "RunIntoFootballPlayers".into(),
                message: "You just ran into football players. Ouch".into(),
                megaEvent: false,
                health: 5,
                ..Default::default()
            }, 3),
            (events {
                name: "RunIntoCar".into(),
                message: "You just ran into a car. Watch out".into(),
                megaEvent: false,
                health: 8,
                ..Default::default()
            }, 4),
            (events {
                name: "RunIntoCactus".into(),
                message: "You just ran into a cactus. Be careful".into(),
                megaEvent: false,
                health: 7,
                ..Default::default()
            }, 5),
            (events {
                name: "LunchAtFrary".into(),
                message: "You are near Frary Dining Hall. Go in for lunch?".into(),
                megaEvent: true,
                ..Default::default()
            }, 6),
            (events {
                name: "CrowdedLunch".into(),
                message: "The lunch is out the door. Literally. You lose time.".into(),
                chance: 7,
                time: 15,
                health: -20,
                hunger: -10,
                ..Default::default()
            }, 7),
            (events {
                name: "FoodPoisoning".into(),
                message: "The food you ate gave you food poisoning. You lose health and time.".into(),
                chance: 1,
                time: 2,
                health: 5,
                hunger: -10,
                ..Default::default()
            }, 8),
            (events {
                name: "GreatLunch".into(),
                message: "The line was not too long. Your hunger is satisfied".into(),
                time: 5,
                chance: 4,
                health: -20,
                hunger: -20,
                ..Default::default()
            }, 9),
            (events {
                name: "EnterFrary".into(),
                message: "You have Entered Frary".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }, 10),
            (events {
                name: "SkipFrary".into(),
                message: "You didn't go for lunch".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }, 11),
            (events {
                name: "ContinueToRun".into(),
                message: "You keep running".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }, 12),
            (events {
                name: "ContinueToSkateboard".into(),
                message: "You keep skateboarding".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }, 13),
        ];
        
        let mut runningOptions = vec![eventsVec[12].0.clone()];
        let mut runningEvents = vec![eventsVec[2].0.clone(),eventsVec[3].0.clone(),eventsVec[4].0.clone()];
        let mut skateOptions = vec![eventsVec[13].0.clone()];
        let mut fraryOptions = vec![eventsVec[10].0.clone(), eventsVec[11].0.clone()];
        let mut enterFraryEvents = vec![eventsVec[7].0.clone(), eventsVec[8].0.clone(), eventsVec[9].0.clone()];
        
        let mut newRunningOptions = &eventsVec.clone()[0].0;
        newRunningOptions.clone().addEvents(runningOptions.clone());
        swap_vec(&mut eventsVec, 0, newRunningOptions.clone());

        let mut newRunningEvents = &eventsVec.clone()[12].0;
        newRunningEvents.clone().addEvents(runningEvents);
        swap_vec(&mut eventsVec, 12, newRunningEvents.clone());

        let mut newSkatingOptions = &eventsVec.clone()[13].0;
        newSkatingOptions.clone().addEvents(runningOptions.clone());
        swap_vec(&mut eventsVec, 13, newSkatingOptions.clone());

        let mut newFraryOptions = &eventsVec.clone()[13].0;
        newFraryOptions.clone().addEvents(fraryOptions);
        swap_vec(&mut eventsVec, 13, newFraryOptions.clone());

        let mut newFraryEvents = &eventsVec.clone()[13].0;
        newFraryEvents.clone().addEvents(enterFraryEvents);
        swap_vec(&mut eventsVec, 13, newFraryEvents.clone());
       

        // eventsVec[6].0.addEvents( fraryOptions);
        // eventsVec[10].0.addEvents( enterFraryEvents);

        // let eventsVec = vec![events {
        //     name: "Running".into(),
        //     message: "You run.".into(),
        //     mega_events: false,
        //     ..Default::default()
        // }];

        // let mut eventsVec = Vec::new();
        // eventsVec.push(events {
        //     name: "Running".into(),
        //     message: "You run.".into(),
        //     megaEvent: false,
        //     ..Default::default()
        // });
        return eventsVec;
    }
    
// 
