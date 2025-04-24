pub enum State {
    Initial,
    Get(Get),
    Set(Set),
    Delete(Delete),
    Ping(Ping),
    Error,
}

pub struct Get {
    pub key: String,
}

pub struct Set {
    pub key: String,
    pub value: String,
}

pub struct Delete {
    pub key: String,
}

pub struct Ping {}


