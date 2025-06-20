pub struct Password {
    pub title: String,
    pub username: String,
    pub password: String,
}

impl Password {
    pub fn new(title: String, username: String, password: String) -> Self {
        Self {
            title,
            username,
            password,
        }
    }
}
