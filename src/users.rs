use uuid::Uuid;

pub(crate) mod controller;
pub(crate) mod profile;

#[derive(Debug, Clone)]
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

pub(crate) enum UserRepositoryError {}

pub(crate) trait UserRepository: Send + Sync {
    async fn insert(&self, user: User) -> Result<User, UserRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Option<User>;
}

#[cfg(test)]
mod tests;
