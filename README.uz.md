# Rust Password Brute Force

Optimallashtirilgan algoritmlardan foydalangan holda Rust dasturlash tilida yozilgan parolni topish vositasi. Server bilan bog'liq muammolar bo'yicha avtomatik qayta urinib ko'rish va jarayonni kuzatish kabi xususiyatlarga ega va tezlik uchun mo'ljallangan.

## 🌟 Xususiyatlari

- 🚀 **Tezkor ishlash** - Optimallashtirilgan algoritm va minimal log yozish
- 🔄 **Avtomatik qayta urinish** - Server bloklanganda device nomini avtomatik o'zgartiradi
- 📋 **Mashhur parollardan boshlash** - Odamlar ko'p ishlatadigan parollarni birinchi navbatda tekshiradi
- 🎯 **Foydalanuvchi tekshiruvi** - Agar foydalanuvchi topilmasa, dastur darhol to'xtaydi
- 📊 **Progress kuzatish** - Har 500 urinishda progress haqida xabar beradi
- 🌐 **Environment sozlamalari** - .env fayli orqali moslashtirish imkoniyati
- 📁 **Fayldan ma'lumotlarni o'qish** - Fake device va parollarni tashqi fayllardan o'qish

## ⚠️ MUHIM OGOHLANTIRISH

**BU DASTUR FAQAT O'QITISH VA RUHSAT ETILGAN XAVFSIZLIK TEKSHIRUVLARI UCHUN MO'LJALLANGAN.**

- ❌ **Ruxsatsiz foydalanish qonuniy emas** - Boshqa serverlarga ruxsatsiz hujum qilish qonuniy emas
- ✅ **Faqat o'zingizning tizimlaringizda ishlating** - Faqat o'zingizga tegishli serverlarda yoki ruxsat olingan joylarda ishlating
- ⚖️ **Javobgarlik** - Ushbu dasturdan noto'g'ri foydalanish natijasida yuzaga keladigan barcha muammolardan foydalanuvchi o'zi javobgardir
- 🔒 **Maxfiylik** - Hech qanday shaxsiy yoki maxfiy ma'lumotlarni tekshirmang

Dasturdan foydalanishdan oldin [LITSENZIYA](LICENSE) va [CONTRIBUTING](CONTRIBUTING.uz.md) hujjatlarini diqqat bilan o'qib chiqing.

## 📦 O'rnatish

1. Rustni o'rnatish (agar yo'q bo'lsa):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Loyihani klonlash:

```bash
git clone <repository-url>
cd password_finder
```

3. Kerakli kutubxonalarni o'rnatish:

```bash
cargo build
```

## ⚙️ Sozlash

### .env faylini yaratish

Loyiha ildizida `.env` faylini yarating va quyidagi sozlamalarni qo'ying:

```env
STARTER_CHAR=@
API_URL=https://example.com/api/login
LOG_LIMIT=500
```

- `STARTER_CHAR`: Foydalanuvchi nomining boshlang'ich belgisi (@)
- `API_URL`: API manzili
- `LOG_LIMIT`: Har qancha urinishdan keyin yangi log fayl ochish

### Ma'lumotlar fayllari

Loyiha ildizida quyidagi fayllarni yaratishingiz mumkin:

1. **fake_devices.txt** - Fake qurilma nomlari ro'yxati
2. **common_passwords.txt** - Mashhur parollar ro'yxati

Agar bu fayllar mavjud bo'lmasa, dastur standart qiymatlardan foydalanadi.

#### fake_devices.txt misoli:

```
Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36
Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36
```

#### common_passwords.txt misoli:

```
password
12345678
qwertyui
admin123
```

## 🚀 Ishlatish

Dasturni ishga tushirish:

```bash
cargo run
```

Dastur sizdan quyidagi ma'lumotlarni so'raydi:

1. **Ishlash usulini tanlang**:

   - `1` - Oddiy (birma-bir)
   - `2` - Asinxron (bir vaqtning o'zida bir nechta so'rov)

2. **Agar asinxron usulni tanlasangiz**:

   - Bir vaqtda nechta so'rov yuborishni kiriting

3. **Davom etish yoki boshidan boshlash**:

   - `b` - Boshidan boshlash
   - `d` - Oxirgi urinishdagi paroldan davom etish

4. **Foydalanuvchi nomini kiriting**

## 📝 Log fayllari

Dastur `password_finder_1.log`, `password_finder_2.log` va hokazo nomli log fayllarini yaratadi. Har 500 urinishda yangi log fayli ochiladi.

Log faylida quyidagi ma'lumotlar saqlanadi:

- Parol topilganda
- Server bloklaganda
- Tarmoq xatolari yuz berganda
- Foydalanuvchi topilmasa

## 🎯 Misol

```bash
$ cargo run
Dastur ishga tushdi
Ishlash usulini tanlang:
1 - Oddiy (birma-bir)
2 - Asinxron (bir vaqtning o'zida bir nechta so'rov)
1
Boshidan boshlaymizmi yoki davom ettiramizmi? (b/d): b
Foydalanuvchi nomini kiriting: admin
@admin uchun parolni topish boshlandi...
500 ta parol sinandi. Hozirgi parol: password12
1000 ta parol sinandi. Hozirgi parol: qwerty123
PAROL TOPILDI! Urinish 1234: Foydalanuvchi: @admin | Parol: admin123 | Response: {"message":"Muvaffaqiyatli kirish","token":"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."}
Dastur muvaffaqiyatli yakunlandi - parol topildi!
```

## 📊 Asinxron ishlash

Agar asinxron usulni tanlasangiz, dastur bir vaqtning o'zida bir nechta so'rov yuboradi, bu tezroq ishlash imkonini beradi:

```bash
$ cargo run
Dastur ishga tushdi
Ishlash usulini tanlang:
1 - Oddiy (birma-bir)
2 - Asinxron (bir vaqtning o'zida bir nechta so'rov)
2
Bir vaqtning o'zida nechta so'rov yuborilsin? 10
10 ta parallel so'rov bilan ishlash boshlandi.
Boshidan boshlaymizmi yoki davom ettiramizmi? (b/d): b
Foydalanuvchi nomini kiriting: testuser
@testuser uchun parolni topish boshlandi...
```

## 🐛 Xatoliklar va muammolar

Agar dastur ishlashda muammo yuzaga kelsa, quyidagilarni tekshiring:

1. Internet aloqasi
2. .env fayli to'g'ri sozlanganligi
3. API manzili to'g'riligi
4. Foydalanuvchi nomi formati (6-15 belgi, @ bilan boshlanishi kerak)

## 🤝 Hissa qo'shish

Loyihaga hissa qo'shmoqchi bo'lsangiz, [CONTRIBUTING.md](CONTRIBUTING.uz.md) faylini diqqat bilan o'qing.

## 📄 Litsenziya

Bu loyiha MIT litsenziyasi ostida chiqarilgan. Batafsil ma'lumot uchun [LICENSE](LICENSE) fayliga qarang.

## ⚠️ Mas'uliyatdan bo'yin tovlash

Ushbu dasturdan foydalanish orqali siz quyidagilarga rozilik bildirasiz:

1. Dastur faqat o'qitish va ruxsat etilgan xavfsizlik testlari uchun ishlatiladi
2. Hech qanday noqonuniy faoliyatda foydalanilmaydi
3. Barcha javobgarlik foydalanuvchi zimmasiga yuklanadi
4. Dastur yaratuvchilari hech qanday noqonuniy faoliyat uchun javobgar emas

---

**ESLATMA**: Bu dastur faqat o'qitish maqsadida yaratilgan. Har qanday noqonuniy faoliyatda foydalanish qat'iyan man etiladi. Dasturdan foydalanishdan oldin mahalliy qonunlar va qoidalarni o'rganib chiqing.

## 📞 Bog'lanish

Agar savollaringiz bo'lsa yoki hissa qo'shmoqchi bo'lsangiz, loyiha sahifasida issue oching yoki pull request yuboring.

---

**Dasturdan foydalanishdan oldin [LITSENZIYA](LICENSE) va [CONTRIBUTING](CONTRIBUTING.uz.md) hujjatlarini diqqat bilan o'qib chiqing.**

---

📖 Mavjud tillar: [English](README.md) | [Oʻzbekcha](README.uz.md) | [Русский](README.ru.md)

---
