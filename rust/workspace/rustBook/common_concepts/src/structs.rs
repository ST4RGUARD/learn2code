struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

impl User {
    pub fn new(username: String, email: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

pub fn my_struct() {
    let mut user1 = User::new(
        "someusername".to_string(),
        "someone@example.com".to_string(),
    );

    user1.email = "changed@example.com".to_string();
    user1.username = "changedusername".to_string();
    user1.active = true;
    user1.sign_in_count = 2;
}
