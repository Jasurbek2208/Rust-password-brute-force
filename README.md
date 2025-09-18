# Password Finder

A fast and efficient Rust program for password recovery. The program attempts to find the correct password for a given username.

## 🌟 Features

- 🚀 **Fast performance** - Optimized algorithm and minimal logging
- 🔄 **Auto retry** - Automatically changes device name when server crashes
- 📋 **Start with popular passwords** - Checks passwords that people use most often first
- 🎯 **User verification** - If the user is not found, the program stops immediately
- 📊 **Progress tracking** - Reports progress every 500 attempts
- 🌐 **Environment settings** - Ability to customize via .env file
- 📁 **Read data from file** - Read fake devices and passwords from external files

## ⚠️ IMPORTANT WARNING

**THIS PROGRAM IS INTENDED FOR EDUCATIONAL AND AUTHORIZED SAFETY INSPECTION PURPOSES ONLY.**

- ❌ **Unauthorized use is not legal** - It is not legal to attack other servers without permission
- ✅ **Only use on your own systems** - Only use on servers owned by you or in places where permission has been obtained
- ⚖️ **Liability** - The user is solely responsible for any problems that may arise from the misuse of this application
- 🔒 **Privacy** - Do not check any personal or confidential information

Please read the [LICENSE](LICENSE) and [CONTRIBUTING](CONTRIBUTING.md) documents carefully before using the program.

## 📦 Installation

1. Install Rust (if not already installed):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Clone the project:

```bash
git clone <repository-url>
cd password_finder
```

3. Install the required libraries:

```bash
cargo build
```

## ⚙️ Setup

### Create a .env file

Create a `.env` file in the project root and set the following settings:

```env
STARTER_CHAR=@
API_URL=https://example.com/api/login
LOG_LIMIT=500
```

- `STARTER_CHAR`: Username starting character (@)
- `API_URL`: API address
- `LOG_LIMIT`: Create a new log file after every attempt

### Data files

You can create the following files in the project root:

1. **fake_devices.txt** - List of fake device names
2. **common_passwords.txt** - List of popular passwords

If these files are not present, the program will use default values.

#### fake_devices.txt example:

```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36
```

#### common_passwords.txt example:

```
password
12345678
qwertyui
admin123
```

## 🚀 Usage

Run the program:

```bash
cargo run
```

The program will ask you for the following information:

1. **Select a run method**:

- `1` - Normal (one by one)
- `2` - Asynchronous (multiple requests at the same time)

2. **If you choose asynchronous method**:

- Enter how many requests to send at a time

3. **Continue or start from the beginning**:

- `b` - Start from the beginning
- `d` - Continue from the password from the last attempt

4. **Enter username**

## 📝 Log files

The program creates log files named `password_finder_1.log`, `password_finder_2.log`, etc. A new log file is opened every 500 attempts.

The following information is stored in the log file:

- When a password is found
- When the server is blocked
- When network errors occur
- When the user is not found

## 🎯 Example

```bash
$ cargo run
The program has started
Select the operation method:
1 - Normal (one by one)
2 - Asynchronous (several requests at the same time)
1
Should we start from the beginning or continue? (b/d): b
Enter username: admin
Starting to find a password for @admin...
500 passwords tried. Current password: password12
1000 passwords tried. Current password: qwerty123
PASSWORD FOUND! Attempt 1234: User: @admin | Password: admin123 | Response: {"message":"Successful login","token":"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."}
The program completed successfully - password found!
```

## 📊 Asynchronous operation

If you choose the asynchronous method, the program sends multiple requests at the same time, which allows you to work faster:

```bash
$ cargo run
The program has started
Select the operation method:
1 - Normal (one by one)
2 - Asynchronous (multiple requests at the same time)
2
How many requests should be sent at the same time? 10
Working with 10 parallel requests has started.
Shall we start from the beginning or continue? (b/d): b
Enter username: testuser
Starting password recovery for @testuser...
```

## 🐛 Bugs and issues

If you have problems with the application, please check the following:

1. Internet connection
2. .env file is configured correctly
3. API address is correct
4. Username format (6-15 characters, must start with @)

## 🤝 Contribute

If you want to contribute to the project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file carefully.

## 📄 License

This project is released under the MIT license. For more information, see the [LICENSE](LICENSE) file.

## ⚠️ Disclaimer

By using this application, you agree to the following:

1. The application is intended for educational and authorized security testing purposes only
2. It is not intended for any illegal activity
3. All responsibility lies with the user
4. The creators of the application are not responsible for any illegal activity

---

**NOTE**: This application is intended for educational purposes only. Any illegal activity is strictly prohibited. Please check your local laws and regulations before using the application.

## 📞 Contact

If you have any questions or would like to contribute, please open an issue or submit a pull request on the project page.

---

**Please read the [LICENSE](LICENSE) and [CONTRIBUTING](CONTRIBUTING.md) documents carefully before using the application.**

---

📖 Available languages: [English](README.md) | [Oʻzbekcha](README.uz.md) | [Русский](README.ru.md)

---
