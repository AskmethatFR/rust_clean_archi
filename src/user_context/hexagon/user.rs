
#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: uuid::Uuid, email: String, password: String) -> Self {
        Self {
            id,
            email,
            password,
        }
    }
}