pub mod events;

pub mod Room {

    use super::events;

    #[derive(Clone)]
    pub enum School {
        HarveyMudd,
        Pomona,
        Pitzer,
        Scripps,
        Cmc,
    }

    #[derive(Clone)]
    pub struct Room {
        pub school: School,
        pub megaEvent: events::events::events,
    }

    impl Room {
        //So what I was imaging was that when we create the rooms, the events are automatically created and attatched to those rooms
        pub fn StartingRoom() -> Room {
            Room {
                school: (School::HarveyMudd),
                megaEvent: (events::events::events::StarterEvent()),
            }
        }

        // pub fn HarveyMudd() -> Room {
        //     Room {
        //         school: (School::HarveyMudd),
        //         megaEvent: (None),
        //         // roomEvents: (Some(events::events::createevents())),
        //     }
        // }

        // pub fn Pomona() -> Room {
        //     Room {
        //         school: (School::Pomona),
        //         megaEvent: (None),
        //         // roomEvents: (Some(events::events::createevents())),
        //     }
        // }

        // pub fn Scripps() -> Room {
        //     Room {
        //         school: (School::Scripps),
        //         megaEvent: (None),
        //         // roomEvents: (Some(events::events::createevents())),
        //     }
        // }

        // pub fn Pitzer() -> Room {
        //     Room {
        //         school: (School::Pitzer),
        //         megaEvent: (None),
        //     }
        // }

        // pub fn Cmc() -> Room {
        //     Room {
        //         school: (School::Cmc),
        //         megaEvent: (None),
        //     }
        // }
    }
}
