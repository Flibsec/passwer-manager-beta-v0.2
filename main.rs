mod new_passw;
use std::fs;
use std::io;
use std::path::Path;
fn main() {
    println!("\x1B[2J\x1B[1;1H");
    let file_exists_key = Path::new("key").exists();
    if file_exists_key {
    } else {
        fs::write("key", "#)7^#@9[2,#572@^6!&558^93@^$*$4#&&7%%&8^^*.93@^$*$4#&&7%%&8^^*.&%477^75^**@$@@@38$32$&*28%23*#$@40'<^5[:9,310").expect("err");
    }
    let file_exists_password = Path::new("password").exists();
    if file_exists_password {
    } else {
        fs::write("password", "").expect("err");
    }
    let key: String = fs::read_to_string("key").expect("err");
    let mut trim_key: String = key.trim().to_string();
    if trim_key.len() == 0 {
        println!(
            "ФАЙЛ С КЛЮЧЁМ БЫЛ СЕРЬЁЗНО ПОВРЕДЖЁН!\nНАЧИНАЕТСЯ ВОСТАНОВЛЕНИЕ КЛЮЧА ПО УМОЛЧАНИЮ..."
        );
        fs::write("key", "#)7^#@9[2,#572@^6!&558^93@^$*$4#&&7%%&8^^*.93@^$*$4#&&7%%&8^^*.&%477^75^**@$@@@38$32$&*28%23*#$@40'<^5[:9,310").expect("err");
        println!("Поздравляю, ключ был успешно сброшен! Перезапустите скрипт.");
        std::process::exit(0);
    } else if trim_key.len() > 12 {
        trim_key.truncate(trim_key.len() - 12);
    } else if trim_key.len() < 12 {
        println!("ФАЙЛ С КЛЮЧЁМ БЫЛ ПОВРЕДЖЁН!\nНАЧИНАЕТСЯ ВОСТАНОВЛЕНИЕ КЛЮЧА ПО УМОЛЧАНИЮ...");
        fs::write("key", "#)7^#@9[2,#572@^6!&558^93@^$*$4#&&7%%&8^^*.93@^$*$4#&&7%%&8^^*.&%477^75^**@$@@@38$32$&*28%23*#$@40'<^5[:9,310").expect("err");
        println!("Поздравляю, ключ был успешно сброшен! Перезапустите скрипт.");
        std::process::exit(0);
    }
    trim_key.remove(11);
    trim_key.remove(9);
    trim_key.remove(7);
    trim_key.remove(6);
    trim_key.remove(5);
    trim_key.remove(4);
    trim_key.remove(3);
    trim_key.remove(1);
    let true_key: String = trim_key
        .to_lowercase()
        .replace("^!$@!#9№@*'9", "a")
        .replace("7^№*2%!88№#85*#&5", "b")
        .replace("6&248#^3!##№9867", "c")
        .replace("6@:7&';8#4@9:4&!%^87", "d")
        .replace("8%85*&@%4**3^72=", "e")
        .replace("*^%@8!9#%6;}*4$#^8", "f")
        .replace("&=:32&5$:&522?95{'93^", "g")
        .replace("&79#*3-?*37*;5}846*", "h")
        .replace("8#4$^5:?^55?4№2=8*&2", "i")
        .replace("$&6396!826%$@92##$9$$2222*", "j")
        .replace("45$$@@5*36$7@75*74^8", "k")
        .replace("$!54^395@^#9#^$@8*^6&^#%9%$$$!9%$@88", "l")
        .replace("@^$4*@@32^5$#89$434#!78*66#73", "m")
        .replace("#49327@4@%6968^5^##&929!$76^928!&8^", "n")
        .replace("93@^$*$4#&&7%%&8^^*.", "o")
        .replace("2!8473&@278#96@$!!@7%", "p")
        .replace("^@$!;%%44№'$3-$!%${*%6", "q")
        .replace("#72#72@^6!&558^", "r")
        .replace("6!3%92^&36**67#636%&@7^7", "s")
        .replace("&%477^75^**@$@@@38$32$&*28%23*#$@4", "t")
        .replace("4@95^!#4535^%3@7&$%24%@$5&%^", "u")
        .replace("&9436$%%#@", "v")
        .replace("^*!3578%!4@92*!*#63@!^4", "w")
        .replace("9*8*&^##", "x")
        .replace("2^@%!69#276&#!!$#*5$**@$7#3!", "y")
        .replace("#6!9@2^*#*!95*#26^8*@@%%*249@*3%$9^76328798%5%#", "z");
    let mut variable_with_key = String::new();
    println!("Введите ключ (по умолчанию 'root')");
    io::stdin().read_line(&mut variable_with_key).expect("err");
    println!("\x1B[2J\x1B[1;1H");
    if variable_with_key.trim().to_lowercase() != true_key {
        println!("\x1B[2J\x1B[1;1H");
        println!("Неверный ключ!");
        std::process::exit(0);
    }
    loop {
        let mut user_choose = String::new();
        println!(
            "Что вы хотите сделать?: \n\t1-поменять ключ,\n\t2-создать новый пароль,\n\t3-посмотреть текущий пароль,\n\t4-закрыть программу"
        );
        io::stdin().read_line(&mut user_choose).expect("err");
        if user_choose.trim() == "1" {
            println!("\x1B[2J\x1B[1;1H");
            create_new_key();
        } else if user_choose.trim() == "2" {
            println!("\x1B[2J\x1B[1;1H");
            let mut new_password_verification = String::new();
            println!("Вы уверенны что хотите создать новый пароль?:");
            io::stdin()
                .read_line(&mut new_password_verification)
                .expect("error");
            loop {
                match new_password_verification.trim() {
                    "yes" | "Yes" | "Да" | "да" | "y" | "д" => {}
                    _ => {
                        println!("\x1B[2J\x1B[1;1H");
                        break;
                    }
                }
                let mut new_password = String::new();
                println!("Введите ваш новый пароль пароль:");
                io::stdin().read_line(&mut new_password).expect("err");
                if new_passw::create_new_password(&mut new_password) == 1 {
                    break;
                }
            }
        } else if user_choose.trim() == "3" {
            println!("\x1B[2J\x1B[1;1H");
            let mut variable_with_password: String = fs::read_to_string("password").expect("err");
            match variable_with_password.trim() {
                "" | " " => {
                    println!("У вас пока нет пароля.");
                }
                _ => {
                    variable_with_password.truncate(variable_with_password.len() - 10); //.truncate() - .pop() только удаляет с конца
                    let not_hex_password =
                        u128::from_str_radix(&variable_with_password, 16).expect("err"); //делаем из hex числа обычное
                    let true_password = not_hex_password - 9268153;
                    println!("У вас уже есть пароль!: {}", true_password);
                }
            }
        } else if user_choose.trim() == "4" {
            println!("\x1B[2J\x1B[1;1H");
            break;
        }
    }
}
fn create_new_key() {
    let mut new_key = String::new();
    println!("Вы уверенны что хотите создать новый ключ?:");
    io::stdin().read_line(&mut new_key).expect("err");
    match new_key.trim() {
        "y" | "yes" | "да" | "д" | "Yes" | "Да" => {
            let mut variable_with_new_key = String::new();
            println!(
                "Введите новый ключ (Учтите он НЕ должен содержать цифры и знаки\nОн может содержать только буквы и пробелы):"
            );
            io::stdin()
                .read_line(&mut variable_with_new_key)
                .expect("err");
            let mut decrypted_variable_with_new_key = String::from(variable_with_new_key.trim())
                .to_uppercase()
                .replace("A", "^!$@!#9№@*'9")
                .replace("B", "7^№*2%!88№#85*#&5")
                .replace("C", "6&248#^3!##№9867")
                .replace("D", "6@:7&';8#4@9:4&!%^87")
                .replace("E", "8%85*&@%4**3^72=")
                .replace("F", "*^%@8!9#%6;}*4$#^8")
                .replace("G", "&=:32&5$:&522?95{'93^")
                .replace("H", "&79#*3-?*37*;5}846*")
                .replace("I", "8#4$^5:?^55?4№2=8*&2")
                .replace("J", "$&6396!826%$@92##$9$$2222*")
                .replace("K", "45$$@@5*36$7@75*74^8")
                .replace("L", "$!54^395@^#9#^$@8*^6&^#%9%$$$!9%$@88")
                .replace("M", "@^$4*@@32^5$#89$434#!78*66#73")
                .replace("N", "#49327@4@%6968^5^##&929!$76^928!&8^")
                .replace("O", "93@^$*$4#&&7%%&8^^*.")
                .replace("P", "2!8473&@278#96@$!!@7%")
                .replace("Q", "^@$!;%%44№'$3-$!%${*%6")
                .replace("R", "#72#72@^6!&558^")
                .replace("S", "6!3%92^&36**67#636%&@7^7")
                .replace("T", "&%477^75^**@$@@@38$32$&*28%23*#$@4")
                .replace("U", "4@95^!#4535^%3@7&$%24%@$5&%^")
                .replace("W", "^*!3578%!4@92*!*#63@!^4")
                .replace("V", "&9436$%%#@")
                .replace("X", "9*8*&^##")
                .replace("Y", "2^@%!69#276&#!!$#*5$**@$7#3!")
                .replace("Z", "#6!9@2^*#*!95*#26^8*@@%%*249@*3%$9^76328798%5%#");
            decrypted_variable_with_new_key.insert(1, ')');
            decrypted_variable_with_new_key.insert(3, '^');
            decrypted_variable_with_new_key.insert(4, '#');
            decrypted_variable_with_new_key.insert(5, '@');
            decrypted_variable_with_new_key.insert(6, '9');
            decrypted_variable_with_new_key.insert(7, '[');
            decrypted_variable_with_new_key.insert(9, ',');
            decrypted_variable_with_new_key.insert(11, '5');
            decrypted_variable_with_new_key.push_str("0'<^5[:9,310");
            fs::write("key", decrypted_variable_with_new_key).expect("err");
        }
        _ => {
            println!("\x1B[2J\x1B[1;1H");
        }
    }
}
