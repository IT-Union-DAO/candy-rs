use crate::value::CandyValue;

pub struct Query {
    name: String,
    next: Vec<Query>,
}

pub enum QueryMode {
    All,
    Some(Vec<Query>),
}

pub struct Update {
    name: String,
    mode: UpdateMode,
}

pub enum UpdateMode {
    Set(CandyValue),
    Lock(CandyValue),
    Next(Vec<Update>),
}

pub struct UpdateRequest {
    id: String,
    update: Vec<Update>,
}
