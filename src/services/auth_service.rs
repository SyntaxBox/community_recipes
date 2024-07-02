pub fn login(name: String, password: String) -> String {
    return "Your name is ".to_owned() + &name + " and your password is " + &password;
}

pub fn register(name: String, password: String) -> String {
    return "Your name is ".to_owned() + &name + " and your password is " + &password;
}

pub fn logout() -> String {
    return "logout".to_owned();
}
