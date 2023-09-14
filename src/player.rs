// use crate::item;
// //Player
#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

//Importing the module Item

// pub mod player {
    // use crate::item;

    use crate::item::item;


    #[derive(Clone)]
    pub struct Player {
        pub name: String,
        pub hunger: i8, // Biggest Number in a u8 is 255
        pub time: i8,   // Maybe make this a float , look at later -agreed Kovit
        pub health: i8,
        pub item: Option<item>,
    }

    impl Default for Player {
        fn default() -> Self {
            Player {
                name: ("".into()),
                hunger: (100),
                time: (100),
                health: (100),
                item: (None),
            }
        }
    }
    impl Player {
        pub fn test(&mut self) {
            println!("{}", self.hunger)
        }

        //Idk what to fuckin call this variable tbh
        pub fn updateHealth(&mut self, incrementer: i8) {
            self.health -= incrementer;
        }
        pub fn updateHunger(&mut self, incrementer: i8) {
            self.hunger -= incrementer;
        }
        pub fn updateTime(&mut self, incrementer: i8) {
            self.time -= incrementer;
        }
    }
// }
