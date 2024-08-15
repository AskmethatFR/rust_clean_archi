mod tests;

use crate::user_context::hexagon::user::User;
use crate::user_context::hexagon::user_repository::UserRepository;

pub  enum CreateUserError {
    UserAlreadyExists,
}
pub struct CreateUserUseCase<'a, T: UserRepository> {
    user_repository: &'a mut T,
}

impl <'a, T: UserRepository>CreateUserUseCase<'a,T> {
    pub fn new(user_repository: &'a mut T) -> Self {
        CreateUserUseCase { user_repository }
    }

    pub fn execute(&mut self, user: User) -> Result<(), CreateUserError> {
        let user_exists = self.user_repository.get_user_by_email(&user.email).is_some();
        
        if user_exists {
            return Err(CreateUserError::UserAlreadyExists);
        }
        
        self.user_repository.add_user(user);
        Ok(())
    }
}
