// pub mod Events {
    use weighted_rand::builder::*;
    use std::clone::Clone;


    #[derive(Clone)]
    pub struct Event {
        name: String,
        message: String,
        mega_event: bool,
        pub events: Vec<Event>,
        chance: u8,                  // make this a float, or choose arbitary max
        time: i16,
        health: i16,
        hunger: i16,
    }

    // implements default values for each events -> let p = events { name: "Lunch", ..Default::default() };
    impl Default for Event {
        fn default() -> Event {
            Event {
                name: "Untitledevents".into(),
                message: "EmptyMessage".into(),
                mega_event: false,
                events: Vec::new(),
                chance: 1,    // lets set a default attributes
                time: 5,
                health: 0,
                hunger: 0,
            }
        }
    }

    impl Event {
        pub fn starter_event() -> Event {
            Event {
                name: "Starting Room".into(),
                message: "Your objective is to make it to your class".into(),
                mega_event: false,
                events: Vec::new(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
            }
        }

        //Implementing Getters, I think it'll make lyfe easier down the line
        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_message(&self) -> &str {
           &self.message
        }

        pub fn get_mega_event(&self) {}

        pub fn get_events(&self) -> &Vec<Event> {
            &self.events
        }

        fn add_events(&mut self, new_events: Vec<Event>) {
            // if let Some(existing_events) = &mut self.events {
            //     existing_events.extend(new_events);
            // } else {
            //     self.events = Some(new_events);
            // }
            self.events.extend(new_events);
        }

        pub fn get_chance(&self) -> u8 {
            self.chance
        }

        pub fn get_time(&self) -> i16 {
            self.time
        }

        pub fn get_health(&self) -> i16 {
            self.health
        }
        pub fn get_hunger(&self) -> i16 {
            self.hunger
        }

        pub fn rand_event(&self) -> i32 {
            if !self.events.is_empty() {

                if self.events.len() == 1 {
                    return 0
                }

                let choices = self.events.clone();
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

    // going to have to restructure a lot of this
    pub fn createevents() -> Vec<Event> {
        //For some reason the above code had an issue with creating two variables then putting it into a vector ??, so i  just took one and put it inside the vector for testing purposses.
        let mut events_vec = vec![
            (Event {
                name: "Running".into(),
                message: "You run.".into(),
                mega_event: true,
                ..Default::default()
            }),
            (Event {
                name: "Skateboarding".into(),
                message: "You skateboard.".into(),
                mega_event: true,
                ..Default::default()
            }),
            (Event {
                name: "RunIntoStairs".into(),
                message: "You just ran into a set of stairs. Ouch".into(),
                mega_event: false,
                health: 10,
                hunger: 10,
                time: 9,
                chance: 8,
                ..Default::default()
            }),
            (Event {
                name: "RunIntoFootballPlayers".into(),
                message: "You just ran into football players. Ouch".into(),
                mega_event: false,
                chance: 4,
                health: 18,
                hunger: 10,
                time: 9,
                ..Default::default()
            }),
            (Event {
                name: "RunIntoCar".into(),
                message: "You just ran into a car. Watch out".into(),
                mega_event: false,
                chance: 2,
                health: 25,
                hunger: 10,
                time: 9,
                ..Default::default()
            }),
            (Event {
                name: "RunIntoCactus".into(),
                message: "You just ran into a cactus. Be careful".into(),
                mega_event: false,
                chance: 6,
                health: 15,
                hunger: 10,
                time: 9,
                ..Default::default()
            }),
            (Event {
                name: "LunchAtFrary".into(),
                message: "You are near Frary Dining Hall. Go in for lunch?".into(),
                mega_event: true,
                ..Default::default()
            }),
            (Event {
                name: "CrowdedLunch".into(),
                message: "The lunch is out the door. Literally. You lose time.".into(),
                chance: 7,
                time: 25,
                health: -20,
                hunger: -20,
                ..Default::default()
            }),
            (Event {
                name: "FoodPoisoning".into(),
                message: "The food you ate gave you food poisoning. You lose health and time.".into(),
                chance: 1,
                time: 20,
                health: 25,
                hunger: -10,
                ..Default::default()
            }),
            (Event {
                name: "GreatLunch".into(),
                message: "The line was not too long. Your hunger is satisfied".into(),
                time: 10,
                chance: 4,
                health: -20,
                hunger: -25,
                ..Default::default()
            }),
            //10
            (Event {
                name: "EnterFrary".into(),
                message: "Enter Frary and get some cafeteria food. Delicious!".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            //11
            (Event {
                name: "SkipFrary".into(),
                message: "Skip the potentially long lunch".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            (Event {
                name: "ContinueToRun".into(),
                message: "Run!".into(),
                chance: 0,
                time: -17,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            (Event {
                name: "ContinueToSkateboard".into(),
                message: "Use skateboard!".into(),
                chance: 0,
                time: -10,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            //14
            (Event {
                name: "TSwift".into(),
                message: "You run into an angry Taylor Swift".into(),
                mega_event: true,
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            (Event {
                name: "RunAwayTSwift".into(),
                message: "Run away!".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            (Event {
                name: "CalmTaylor".into(),
                message: "Try to calm her down by singing your favorite Taylor Swift song".into(),
                chance: 0,
                time: 0,
                health: 0,
                hunger: 0,
                ..Default::default()
            }),
            (Event {
                name: "SuccessRunAwayTSwift".into(),
                message: "You successfully run away".into(),
                chance: 3,
                time: 5,
                health: 5,
                hunger: 5,
                ..Default::default()
            }),
            (Event {
                name: "CarbonFootPrint".into(),
                message: "She uses her private plane to catch up to you. Her extra carbon footprint effects you immensely.".into(),
                chance: 2,
                time: 15,
                health: 25,
                hunger: 5,
                ..Default::default()
            }),
            (Event {
                name: "LoveSing".into(),
                message: "She loves your singing. You walk together, time becomes a Blank Space.".into(),
                chance: 3,
                time: 0,
                health: 8,
                hunger: 8,
                ..Default::default()
            }),
            (Event {
                name: "HateSing".into(),
                message: "She hates your singing. She starts singing a bad song about you. Her fans boo you.".into(),
                chance: 8,
                time: 20,
                health: 10,
                hunger: 5,
                ..Default::default()
            }),
            (Event {
                name: "NoDamage".into(),
                message: "You don't hit anyone. Great job!".into(),
                chance: 3,
                time: 5,
                health: 5,
                hunger: 5,
                ..Default::default()
            }),
            // 22
            (Event {
                name: "HungerFromFrary".into(),
                message: "You don't hit anyone. Great job!".into(),
                chance: 3,
                time: 5,
                health: 15,
                hunger: 30,
                ..Default::default()
            }),
        ];
    
    let running_options = vec![events_vec[12].clone(), events_vec[13].clone()];
    let running_events = vec![events_vec[2].clone(),events_vec[3].clone(),events_vec[4].clone(), events_vec[find_event_index(&events_vec, "NoDamage") as usize].clone()];
    
    let taylor_options = vec![events_vec[15].clone(), events_vec[16].clone()];
    let taylor_calm = vec![events_vec[19].clone(), events_vec[20].clone()];
    let taylor_run = vec![events_vec[17].clone(), events_vec[18].clone()];

    let frary_options = vec![events_vec[10].clone(), events_vec[11].clone()];
    let in_frary = vec![events_vec[7].clone(), events_vec[8].clone(), events_vec[9].clone()];
    let leave_frary = vec![events_vec[22].clone()];
        
    

    add_events_to_event(&mut events_vec, "Running", &running_options);
    add_events_to_event(&mut events_vec, "ContinueToRun", &running_events);
    add_events_to_event(&mut events_vec, "ContinueToSkateboard", &running_events);

    add_events_to_event(&mut events_vec, "TSwift", &taylor_options);
    add_events_to_event(&mut events_vec, "CalmTaylor", &taylor_calm);
    add_events_to_event(&mut events_vec, "RunAwayTSwift", &taylor_run);

    add_events_to_event(&mut events_vec, "LunchAtFrary", &frary_options);
    add_events_to_event(&mut events_vec, "EnterFrary", &in_frary);
    add_events_to_event(&mut events_vec, "SkipFrary", &taylor_run);

        return events_vec;
    }

    fn add_events_to_event(events_vec: &mut Vec<Event>, event_name: &str, events_to_add: &Vec<Event>) {
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

    pub fn find_event_index(events_vec: &Vec<Event>, event_name: &str) -> i8 {
        // Find the index of the target event by name
        if let Some(index) = events_vec.iter().position(|event| event.name == event_name) {
            index as i8
    
        } else {
            -1
        }
    }
// 
