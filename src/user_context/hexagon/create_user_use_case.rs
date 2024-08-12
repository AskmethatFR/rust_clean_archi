mod tests;

use crate::user_context::hexagon::user::User;
use crate::user_context::hexagon::user_repository::UserRepository;

pub struct CreateUserUseCase<'a, T: UserRepository> {
    user_repository: &'a mut T,
}

impl <'a, T: UserRepository>CreateUserUseCase<'a,T> {
    pub fn new(user_repository: &'a mut T) -> Self {
        CreateUserUseCase { user_repository }
    }

    pub fn execute(&mut self, user: User) -> () {
        self.user_repository.add_user(user);
    }
}
