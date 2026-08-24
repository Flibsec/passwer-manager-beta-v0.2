use std::fs;
pub fn create_new_password(new_password: &mut str) -> u8 {
    match new_password.trim().parse::<u128>() {
        Ok(encrypted_password) => {
            println!("Вот ваш пароль!: {}", encrypted_password);
            let mut decrypted_password: String = format!("{:x}", encrypted_password + 9268153);
            decrypted_password.push_str("1f6ac3f29d");
            fs::write("password", decrypted_password).expect("error");
            1
        }
        Err(error) => {
            print!("\x1B[2J\x1B[1;1H");
            println!("Не удалось считать ваш пароль, ошибка: {}", error);
            0
        }
    }
}
