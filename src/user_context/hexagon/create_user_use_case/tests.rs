#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::user_context::hexagon::create_user_use_case::CreateUserUseCase;
    use crate::user_context::hexagon::user::User;
    use crate::user_context::hexagon::user_repository::UserRepository;
    use crate::user_context::secondary_adapters::in_memory_user_repository::InMemoryUserRepository;
    
    #[test]
    fn test_create_user_use_case() {
        // Arrange
        let mut repo = InMemoryUserRepository::new();
        let user = User::new(Uuid::new_v4(), "JohnDoe@gmail.com".to_string(), "A123456".to_string());

        let mut use_case = CreateUserUseCase::new(&mut repo);
        let _ = use_case.execute(user.clone());

        let all_users = repo.all();


        assert_eq!(all_users.len(), 1);
        assert_eq!(all_users[0].id, user.id);
        assert_eq!(all_users[0].email, user.email);
        assert_eq!(all_users[0].password, user.password);
    }
    
    #[test]
    fn user_already_exists() {
        // Arrange
        let mut repo = InMemoryUserRepository::new();
        let user = User::new(Uuid::new_v4(), "toto@gmail.com".to_string(), "A123456".to_string());

        repo.add_user(user.clone());

        let mut use_case = CreateUserUseCase::new(&mut repo);
        let result = use_case.execute(user.clone());

        assert_eq!(result.is_err(), true);
    }
}

