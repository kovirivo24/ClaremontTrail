// pub mod events {
    use weighted_rand::builder::*;
    use std::{clone::Clone, ops::Deref};


    #[derive(Clone)]
    pub struct events {
        name: String,
        message: String,
        megaEvent: bool,
        pub events: Vec<events>,
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
                events: Vec::new(),
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
                events: Vec::new(),
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

        pub fn get_events(&self) -> &Vec<events> {
            &self.events
        }

        fn add_events(&mut self, new_events: Vec<events>) {
            // if let Some(existing_events) = &mut self.events {
            //     existing_events.extend(new_events);
            // } else {
            //     self.events = Some(new_events);
            // }
            self.events.extend(new_events);
        }

        pub fn getChance(&self) -> u8 {
            (self.chance)
        }

        pub fn getTime(&self) -> i8 {
            (self.time)
        }

        pub fn getHealth(&self) -> i8 {
            (self.health)
        }
        pub fn getHunger(&self) -> i8 {
            (self.hunger)
        }

        // pub fn randEvent(&self) -> i32 {
        //     if self.events.is_some() {

        //         let choices = self.events.clone().unwrap();
        //         let mut weights = Vec::new();
        //         let mut total_weight = 0;
        //         for cur_event in choices {
        //             weights.push(cur_event.chance as u32);
        //             total_weight += cur_event.chance as u32;
        //         }
        //         let builder = WalkerTableBuilder::new(&weights);
        //         let wa_table = builder.build();

        //         for i in (0..total_weight).map(|_| wa_table.next()) {
        //             total_weight = i as u32;
        //         }

        //         return total_weight as i32
                
        //     } else {
        //         return -1
        //     }
        // }
    }

    // fn swap_vec(vec: &mut Vec<(events, i32)>, eventID: i32, variant: events) {
    //     if let Some(item) = vec.iter_mut().find(|item| item.1 == eventID) {
    //         let mut temp = (events::StarterEvent(), eventID); // create placeholder value
    //         std::mem::swap(item, &mut temp); // swap the placeholder in
    //         *item = (variant, eventID); // overwrite with real value
    //     }
    // }
    // going to have to restructure a lot of this
    pub fn createevents() -> Vec<events> {
        //For some reason the above code had an issue with creating two variables then putting it into a vector ??, so i  just took one and put it inside the vector for testing purposses.
        let mut eventsVec = vec![
            (events {
                name: "Running".into(),
                message: "You run.".into(),
                megaEvent: true,
                ..Default::default()
            }),
            (events {
                name: "Skateboarding".into(),
                message: "You skateboard.".into(),
                megaEvent: true,
                ..Default::default()
            }),
            (events {
                name: "RunIntoStairs".into(),
                message: "You just ran into a set of stairs. Ouch".into(),
                megaEvent: false,
                health: 2,
                ..Default::default()
            }),
            (events {
                name: "RunIntoFootballPlayers".into(),
                message: "You just ran into football players. Ouch".into(),
                megaEvent: false,
                health: 5,
                ..Default::default()
            }),
            (events {
                name: "RunIntoCar".into(),
                message: "You just ran into a car. Watch out".into(),
                megaEvent: false,
                health: 8,
                ..Default::default()
            }),
            (events {
                name: "RunIntoCactus".into(),
                message: "You just ran into a cactus. Be careful".into(),
                megaEvent: false,
                health: 7,
                ..Default::default()
            }),
            (events {
                name: "LunchAtFrary".into(),
                message: "You are near Frary Dining Hall. Go in for lunch?".into(),
                megaEvent: true,
                ..Default::default()
            }),
            (events {
                name: "CrowdedLunch".into(),
                message: "The lunch is out the door. Literally. You lose time.".into(),
                chance: 7,
                time: 15,
                health: -20,
                hunger: -10,
                ..Default::default()
            }),
            (events {
                name: "FoodPoisoning".into(),
                message: "The food you ate gave you food poisoning. You lose health and time.".into(),
                chance: 1,
                time: 2,
                health: 5,
                hunger: -10,
                ..Default::default()
            }),
            (events {
                name: "GreatLunch".into(),
                message: "The line was not too long. Your hunger is satisfied".into(),
                time: 5,
                chance: 4,
                health: -20,
                hunger: -20,
                ..Default::default()
            }),
            (events {
                name: "EnterFrary".into(),
                message: "You have Entered Frary".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            (events {
                name: "SkipFrary".into(),
                message: "You didn't go for lunch".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            (events {
                name: "ContinueToRun".into(),
                message: "You keep running".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            (events {
                name: "ContinueToSkateboard".into(),
                message: "You keep skateboarding".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
        ];
    
    let mut runningOptions = vec![eventsVec[12].clone()];
    let mut runningEvents = vec![eventsVec[2].clone(),eventsVec[3].clone(),eventsVec[4].clone()];
    
    add_events_to_event(&mut eventsVec, "Running", &runningOptions);
    add_events_to_event(&mut eventsVec, "ContinueToRun", &runningEvents);

        return eventsVec;
    }

    fn add_events_to_event(events_vec: &mut Vec<events>, event_name: &str, events_to_add: &Vec<events>) {
        // Find the index of the target event by name
        if let Some(index) = events_vec.iter().position(|event| event.name == event_name) {
            // Clone the target event
            let mut cloned_target = events_vec[index].clone();
    
            // Add the events from events_to_add to the cloned_target's events
            cloned_target.add_events(events_to_add.clone());
    
            // Swap the modified cloned_target back into the events_vec
            std::mem::swap(&mut events_vec[index], &mut cloned_target);
        }
    }

    pub fn find_event_index(events_vec: &Vec<events>, event_name: &str) -> i8 {
        // Find the index of the target event by name
        if let Some(index) = events_vec.iter().position(|event| event.name == event_name) {
            index as i8
    
        } else {
            -1
        }
    }
// 
