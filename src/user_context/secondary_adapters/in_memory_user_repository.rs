use std::collections::HashMap;

use crate::user_context::hexagon::user::User;
use crate::user_context::hexagon::user_repository::UserRepository;

#[derive(Clone)]
pub struct InMemoryUserRepository {
    users: HashMap<String, User>,
    all: Vec<User>,
}

impl InMemoryUserRepository {
    pub(crate) fn new() -> Self {
        InMemoryUserRepository {
            users: HashMap::new(),
            all: vec![],
        }
    }

    pub fn all(&self) -> Vec<User> {
        self.all.clone()
    }
}

impl UserRepository for InMemoryUserRepository {
    fn add_user(&mut self, user: User) -> () {
        self.users.insert(user.id.to_string(), user.clone());
        self.all.push(user);
    }
}