use crate::world::model::WorldModel;

const EVENT_MESSAGE: &str = "";
const FLAG: i16 = 0;

pub const WORLDS: &[WorldModel] = &[
    WorldModel {
        name: "Scania",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8585,
    },
    WorldModel {
        name: "Bera",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8686,
    },
    WorldModel {
        name: "Broa",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8787,
    },
    WorldModel {
        name: "Windia",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8888,
    },
    WorldModel {
        name: "Khaini",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 8989,
    },
    WorldModel {
        name: "Mardia",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9090,
    },
    WorldModel {
        name: "Yellonde",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9191,
    },
    WorldModel {
        name: "Bellocan",
        event_message: EVENT_MESSAGE,
        flag: FLAG,
        base_port: 9292,
    },
];
