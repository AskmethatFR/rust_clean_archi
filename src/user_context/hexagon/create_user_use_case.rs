mod tests;

use std::sync::{Arc, Mutex};

use crate::user_context::hexagon::user::User;
use crate::user_context::hexagon::user_repository::UserRepository;

#[derive(Clone)]
pub struct CreateUserUseCase<T: UserRepository> {
    user_repository: Arc<Mutex<T>>,
}
impl <T: UserRepository>CreateUserUseCase<T> {
    pub fn new(user_repository: Arc<Mutex<T>>) -> Self {
        CreateUserUseCase { user_repository }
    }


    pub fn execute(&self, user: User) -> () {
        let mut repo = self.user_repository.lock().unwrap();
        repo.add_user(user);
    }
}