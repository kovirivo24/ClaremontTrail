// use crate::item;
// //Player
#![allow(dead_code, unused_variables)] // Attribute to hide warnings for unused code

// pub use crate::item;
pub mod item;

pub mod player {

    pub struct Player {
        pub name: String,
        pub hunger: u8, // Biggest Number in a u8 is 255
        pub time: u8,   // Maybe make this a float , look at later -agreed Kovit
        pub health: u8,
        pub item: crate::item,
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
}
