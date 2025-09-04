# Password Finder (Brute Force)

[![Rust](https://img.shields.io/badge/Rust-1.60%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A fast and efficient Rust utility for password recovery.

## 🌟 Features

- 🚀 **High performance** - Optimized algorithm and minimal logging
- 🔄 **Auto-retry** - Automatically changes device name when blocked
- 📋 **Common passwords first** - Checks frequently used passwords first
- 🎯 **User validation** - Stops immediately if user not found
- 📊 **Progress tracking** - Shows progress every 100 attempts

## 📦 Installation

```bash
git clone <repository-url>
cd password_finder
```

## ⚙️ Requirements

- Rust 1.60+
- Tokio
- Reqwest
- Rand

## 🚀 Usage

Run the program:

```bash
cargo run
```

You will be prompted with:

1. **Start from beginning or continue? (b/d)** -

   - `b` - Start password search from beginning
   - `d` - Continue from last attempted password

2. **Enter last attempted password** - (only if you chose `d`)

   - Enter the last password from previous session

3. **Enter username** -
   - Enter username (@ symbol added automatically)

## 📝 Log File

The program writes only important messages to `password_finder.log`:

- When password is found
- When server blocks requests
- Network errors occur
- If user is not found

## 🎯 Example

```bash
$ cargo run
Program started
Start from beginning or continue? (b/d): b
Enter username: admin
Password search started for @admin...
100 passwords attempted. Current password: password12
200 passwords attempted. Current password: qwerty123
PASSWORD FOUND! Password for @admin: admin123
Program completed successfully - password found!
```

## ⚠️ Security Notice

**WARNING:** This tool is intended for authorized testing only.

🚫 Unauthorized use against systems is illegal.

✅ Use only on systems you own or have permission to test.

## 📄 License

MIT License - see LICENSE file for details

## ❓ Support

If you have questions, open an issue or submit a pull request.

---

**Note:** This tool is for educational and authorized security testing purposes only.

---

📖 Available languages: [English](README.md) | [Oʻzbekcha](README.uz.md) | [Русский](README.ru.md)

---
