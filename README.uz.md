# Parol Topuvchi (Brute Force)

[![Rust](https://img.shields.io/badge/Rust-1.60%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Parolni topish uchun tezkor va samarali Rust dasturi.

## 🌟 Xususiyatlari

- 🚀 **Tezkor ishlash** - Optimallashtirilgan algoritm va minimal log yozish
- 🔄 **Avtomatik qayta urinish** - Server bloklaganda device nomini avtomatik o'zgartiradi
- 📋 **Mashhur parollardan boshlash** - Odamlar ko'p ishlatadigan parollarni birinchi navbatda tekshiradi
- 🎯 **Foydalanuvchi tekshiruvi** - Agar foydalanuvchi topilmasa, dastur darhol to'xtaydi
- 📊 **Progress kuzatish** - Har 100 urinishda progress ko'rsatiladi

## 📦 O'rnatish

```bash
git clone <repository-url>
cd password_finder
```

## ⚙️ Talablar

- Rust 1.60+
- Tokio
- Reqwest
- Rand

## 🚀 Ishlatish

Dasturni ishga tushirish:

```bash
cargo run
```

Dastur sizdan quyidagilarni so'raydi:

1. **Boshidan boshlaymizmi yoki davom ettiramizmi? (b/d)** -

   - `b` - Parol qidiruvini boshidan boshlash
   - `d` - Oxirgi urinishdagi paroldan davom etish

2. **Oxirgi urinishdagi parolni kiriting** - (faqat `d` tanlaganingizda)

   - Oldingi sessiyadagi oxirgi parolni kiriting

3. **Foydalanuvchi nomini kiriting** -
   - Foydalanuvchi nomini kiriting (@ belgisi avtomatik qo'shiladi)

## 📝 Log fayl

Dastur `password_finder.log` fayliga faqat muhim xabarlarni yozadi:

- Parol topilganda
- Server bloklaganda
- Tarmoq xatolari yuz berganda
- Foydalanuvchi topilmasa

## 🎯 Misol

```bash
$ cargo run
Dastur ishga tushdi
Boshidan boshlaymizmi yoki davom ettiramizmi? (b/d): b
Foydalanuvchi nomini kiriting: admin
@admin uchun parolni topish boshlandi...
100 ta parol sinandi. Hozirgi parol: password12
200 ta parol sinandi. Hozirgi parol: qwerty123
PAROL TOPILDI! @admin uchun parol: admin123
Dastur muvaffaqiyatli yakunlandi - parol topildi!
```

## ⚠️ Xavfsizlik Eslatmasi

**DIQQAT:** Bu dastur faqat ruxsat etilgan testlash uchun mo'ljallangan.

🚫 Ruxsatsiz tizimlarga hujum qilish qonuniy emas.

✅ Faqat o'zingizning tizimlaringizda yoki ruxsat olingan serverlarda ishlating.

## 📄 Litsenziya

MIT Litsenziyasi - batafsil ma'lumot uchun LICENSE faylini ko'ring

## ❓ Yordam

Agar savollaringiz bo'lsa, issue oching yoki pull request yuboring.

---

**Eslatma:** Bu dastur faqat ta'lim va ruxsat etilgan xavfsizlik testlari uchun mo'ljallangan.

---

📖 Mavjud tillar: [English](README.md) | [Oʻzbekcha](README.uz.md) | [Русский](README.ru.md)

---
