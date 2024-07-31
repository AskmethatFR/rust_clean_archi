use crate::user_context::hexagon::user::User;

pub trait UserRepository: Send + Sync {
    fn add_user(&mut self, user: User);
}