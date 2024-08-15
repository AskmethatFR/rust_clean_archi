use crate::user_context::hexagon::user::User;

pub trait UserRepository: Send + Sync {
    fn add_user(&mut self, user: User);
    fn get_user_by_email(&self, email: &str) -> Option<User>;
}