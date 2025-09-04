use std::fs;

pub fn url_to_key(url: String) -> Option<String> {
    let split: Vec<&str> = url.trim().split("&authKey=").collect();
    let key = String::from(*split.last()?);
    match set_key(&key) {
        Ok(_) => Some(key),
        Err(_) => None,
    }
    
}

pub fn get_key() -> std::io::Result<String> {
    fs::read_to_string("authKey.txt")
}

pub fn set_key(key: &String) -> std::io::Result<()> {
    fs::write("authKey.txt", key)?;
    Ok(())
    // if let Ok(auth_key) = read_to_string("authKey.txt")
}