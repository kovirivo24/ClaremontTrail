// use crate::item;
// //Player
#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

//Importing the module Item

// pub mod player {
    // use crate::item;

    use crate::item::item;


    pub struct Player {
        pub name: String,
        pub hunger: u8, // set to 100
        pub time: u8,   // max 100, decrement, cannot increase
        pub health: u8, // set to 100
        pub item: item,
    }

    impl Player {
        pub fn test(&mut self) {
            println!("{}", self.hunger)
        }

        //Idk what to fuckin call this variable tbh
        pub fn updateHealth(&mut self, incrementer: u8) {
            self.health -= incrementer;
        }
        pub fn updateHunger(&mut self, incrementer: u8) {
            self.hunger -= incrementer;
        }
        pub fn updateTime(&mut self, incrementer: u8) {
            self.time -= incrementer;
        }

        pub fn printStats(self) {
            println!(
                "{} has {}health,{} hunger, and {} hours remaining",
                self.name, self.health, self.hunger, self.time,
            );
        }
    }
// }
