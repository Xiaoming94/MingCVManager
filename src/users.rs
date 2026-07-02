use uuid::Uuid;

pub(crate) mod controller;
pub(crate) mod profile;

#[derive(Debug)]
struct User {
    id: Uuid,
    username: String,
}

impl User {
    fn new(id: Uuid, username: &str) -> Self {
        User {
            id: id,
            username: username.to_string(),
        }
    }
}

#[cfg(test)]
mod tests;
