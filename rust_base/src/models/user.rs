pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

impl User {
    //定义方法
    pub fn set_user(username: String) -> User {
        User {
            active: true,
            username: username,
            email: String::from("longfuchu@163.com"),
            sign_in_count: 22,
        }
    }
}
