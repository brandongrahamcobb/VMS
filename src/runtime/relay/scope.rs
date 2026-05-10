pub enum Scope {
    Map(MapScope),
    Channel(ChannelScope),
    World,
    Local,
    Global,
}

pub enum MapScope {
    SameChannelSameWorld,
    AllChannelsSameWorld,
    AllChannelsAllWorlds,
}

pub enum ChannelScope {
    SameWorld,
    AllWorlds,
}
