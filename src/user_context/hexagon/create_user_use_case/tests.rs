#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use uuid::Uuid;

    use crate::user_context::hexagon::create_user_use_case::CreateUserUseCase;
    use crate::user_context::hexagon::user::User;
    use crate::user_context::secondary_adapters::in_memory_user_repository::InMemoryUserRepository;
    
    #[test]
    fn test_create_user_use_case() {
        // Arrange
        let repo = Arc::new(Mutex::new(InMemoryUserRepository::new()));
        let user = User::new(Uuid::new_v4(), "JohnDoe@gmail.com".to_string(), "A123456".to_string());

        let use_case = CreateUserUseCase::new(Arc::clone(&repo));
        let _ = use_case.execute(user.clone());

        let repo = repo.lock().unwrap();
        let all_users = repo.all();


        assert_eq!(all_users.len(), 1);
        assert_eq!(all_users[0].id, user.id);
        assert_eq!(all_users[0].email, user.email);
        assert_eq!(all_users[0].password, user.password);
    }
}

