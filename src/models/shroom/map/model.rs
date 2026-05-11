#[derive(Clone)]
pub struct MapModel {
    pub wz_id: i32,
}

#[derive(Clone)]
pub struct Map {
    pub model: MapModel,
    pub portals: Vec<Portal>,
}

#[derive(Clone)]
pub struct PortalModel {
    pub pid: i16,
    pub pn: String,
    pub tm: i32,
    pub tn: String,
}

#[derive(Clone)]
pub struct Portal {
    pub model: PortalModel,
}
