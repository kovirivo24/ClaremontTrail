// extern crate rand;
// use rand::{thread_rng, Rng};

// pub mod Item {
    #[derive(Clone)]
    pub struct Item {
        pub name: String,
        pub time_mult: f32,    // for skateboard
        pub hunger_effect: i16, // e.g. snack
        pub health_effect: i16, // e.g. someone poisoined ur snack :(
    }

    // impl Default for Item {
    //     fn default() -> Item {
    //         Item {
    //             name: "UntitledItem".into(),
    //             time_mult: 1.0,   // for skateboard
    //             hunger_effect: 0, // e.g. snack
    //             health_effect: 0, // e.g. someone poisoined ur snack :(
    //         }
    //     }
    // }

    // impl Item {
        pub fn skateboard() -> Item {
            Item {
                name: "Skateboard".into(),
                time_mult: 1.5,
                hunger_effect: -10, // Uses more hunger cause exerting more energy technically lmao
                health_effect: 0,
            }
        }