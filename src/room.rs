pub mod Events;

pub mod Rooms {
    pub enum School {
        HarveyMudd,
        Pomona,
        Pitzer,
        Scripps,
        Cmc,
    }

    pub struct Room {
        school: School,
        megaEvent: Option<Events>,
        events: Option<Vec<Events>>, // Max 1-3
    }

    impl Room {
        pub fn HarveyMudd() -> Room {

            Room { school: (School::HarveyMudd), megaEvent: (None), events: (None) }
        }
    }

}
