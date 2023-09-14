// extern crate rand;
// use rand::{thread_rng, Rng};

pub mod item {
    #[derive(Clone)]
    pub struct item {
        pub name: String,
        pub time_mult: f32,    // for skateboard
        pub hunger_effect: i8, // e.g. snack
        pub health_effect: i8, // e.g. someone poisoined ur snack :(
    }

    impl Default for item {
        fn default() -> item {
            item {
                name: "Untitleditem".into(),
                time_mult: 1.0,   // for skateboard
                hunger_effect: 0, // e.g. snack
                health_effect: 0, // e.g. someone poisoined ur snack :(
            }
        }
    }

    impl item {
        pub fn skateboard() -> item {
            item {
                name: "Skateboard".into(),
                time_mult: 1.5,
                hunger_effect: -10, // Uses more hunger cause exerting more energy technically lmao
                health_effect: 0,
            }
        }

        pub fn snack() -> item {
            // let number: usize = thread_rng().gen_range(0..10); -- this is for rng but it wasnt workign at the time of me writing this lmao
            let number = 2;
            let healthEffect: i8;
            //Leaving it up to RNG if it's experied or not
            if number > 5 {
                healthEffect = 10;
            } else {
                healthEffect = -10
            }
            item {
                name: "Poptart".into(),
                time_mult: 0.0,
                hunger_effect: 10,
                health_effect: healthEffect,
            }
        }
    }
}
