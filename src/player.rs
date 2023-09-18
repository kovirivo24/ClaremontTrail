// use crate::Item;
// //Player
#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

//Importing the module Item

// pub mod player {
    // use crate::Item;

    use crate::item::Item;


    #[derive(Clone)]
    pub struct Player {
        pub name: String,
        pub hunger: i16, // Biggest Number in a u8 is 255
        pub time: i16,   // Maybe make this a float , look at later -agreed Kovit
        pub health: i16,
        pub item: Option<Item>,
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

        pub fn update_health(&mut self, incrementer: i16) {
            self.health -= incrementer;
            if self.health > 100 {
                self.health = 100;
            }
            if self.health < 0 {
                self.health = 0;
            }
        }
        pub fn update_hunger(&mut self, incrementer: i16) {
            self.hunger -= incrementer;
            if self.hunger > 100 {
                self.hunger = 100;
            }
            if self.hunger < 0 {
                self.hunger = 0;
            }
        }
        pub fn update_time(&mut self, incrementer: i16) {
            self.time -= incrementer;
        }
    }
// }
