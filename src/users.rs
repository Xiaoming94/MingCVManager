#[derive(Debug, Clone)]
struct User {
    username: String,
    personal_name: String,
    password: String,
    profile: String,
}

impl User {
    fn new(username: String, personal_name: String, password: String) -> Self {
        User {
            username: username,
            personal_name: personal_name,
            password: password,
            profile: String::new(),
        }
    }
}

#[cfg(test)]
mod tests;
