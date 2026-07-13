use uuid::Uuid;

pub(crate) mod controller;
pub(crate) mod profile;

#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug)]
pub(crate) enum UserRepositoryError {
    UserExist(Uuid),
}

trait UserRepository: Send + Sync {
    async fn insert(&self, user: User) -> Result<User, UserRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Option<User>;
}

#[cfg(test)]
mod tests;
