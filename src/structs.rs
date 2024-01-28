#[derive(Clone, Debug)]
pub struct UserData {
    pub username: String,
    pub password: String,
    pub exptime: String,
    pub roll: String,
}

impl UserData {
    pub fn new(username: String, password: String, exptime: String, roll: String) -> UserData {
        UserData {
            username,
            password,
            exptime,
            roll,
        }
    }
}
