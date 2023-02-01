use crate::unstable::values::CandyValueUnstable;

pub struct UpdateUnstable {
    name: String,
    mode: UpdateModeUnstable,
}

pub enum UpdateModeUnstable {
    Set(CandyValueUnstable),
    Lock(CandyValueUnstable),
    Next(Box<[UpdateUnstable]>),
}


pub struct UpdateRequestUnstable {
    id: String,
    update: Box<[UpdateUnstable]>,
}

pub type DataChunk = CandyValueUnstable;

pub type DataZone = Vec<DataChunk>;

pub type Workspace = Vec<DataZone>;


