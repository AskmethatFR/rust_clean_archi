mod tests;

use std::sync::{Arc, Mutex};

use crate::user_context::hexagon::user::User;
use crate::user_context::hexagon::user_repository::UserRepository;
use crate::user_context::secondary_adapters::in_memory_user_repository::InMemoryUserRepository;

#[derive(Clone)]
pub struct CreateUserUseCase {
    user_repository: Arc<Mutex<InMemoryUserRepository>>,
}
impl CreateUserUseCase {
    pub fn new(user_repository: Arc<Mutex<InMemoryUserRepository>>) -> Self {
        CreateUserUseCase { user_repository }
    }


    pub fn execute(&self, user: User) -> () {
        let mut repo = self.user_repository.lock().unwrap();
        repo.add_user(user.clone());
    }
}