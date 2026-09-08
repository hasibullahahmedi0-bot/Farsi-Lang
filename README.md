# 🌟 زبان فارسی (Farsi-Lang)

**زبان برنامه‌نویسی فارسی قدرتمند برای توسعه وب، برنامه‌های دسک‌تاپ، علم داده و اسکریپت‌نویسی**

![Status](https://img.shields.io/badge/Status-Early%20Development-red)
![License](https://img.shields.io/badge/License-MIT-blue)
![Language](https://img.shields.io/badge/Language-Farsi-green)

---

## 📋 مقدمه

**زبان فارسی** یک زبان برنامه‌نویسی جدید و قدرتمند است که با **سینتکس فارسی** طراحی‌شده و می‌تواند:

✅ برنامه‌های وب (Backend & Frontend)  
✅ برنامه‌های دسک‌تاپ  
✅ اسکریپت‌ها و اتوماسیون  
✅ علم داده و تحلیل  
✅ API و سرویس‌های وب  

---

## 🎯 ویژگی‌های اصلی

- 🔤 **سینتکس فارسی**: کد را به فارسی بنویسید
- ⚡ **سرعت بالا**: Compiled به کد ماشینی
- 📚 **کتابخانه‌های غنی**: توابع و ماژول‌های آماده
- 🌐 **پشتیبانی وب**: توسعه Backend و Frontend
- 💪 **قوی و ایمن**: Type checking و Error handling
- 🚀 **کارایی بالا**: برای Big Data و ML

---

## 📁 ساختار پروژه

```
Farsi-Lang/
├── docs/                 # مستندات کامل
│   ├── syntax.md        # تعریف سینتکس
│   ├── tutorial.md      # آموزش ابتدایی
│   └── api.md           # مرجع API
├── src/                 # کد منبع Compiler
│   ├── lexer.rs         # Tokenizer
│   ├── parser.rs        # Parser
│   ├── compiler.rs      # Compiler
│   └── runtime.rs       # Runtime
├── stdlib/              # کتابخانه استاندارد
│   ├── io/              # ورودی/خروجی
│   ├── math/            # ریاضیات
│   ├── string/          # رشته‌ها
│   └── web/             # توسعه وب
├── examples/            # نمونه‌های کد
├── tests/               # تست‌ها
├── Cargo.toml           # وابستگی‌ها (Rust)
└── README.md            # این فایل
```

---

## 🔧 نصب و استفاده

### نیازمندی‌ها
- Rust 1.70+
- Cargo

### کامپایل کردن

```bash
git clone https://github.com/hasibullahahmedi0-bot/Farsi-Lang.git
cd Farsi-Lang
cargo build --release
```

### اجرا

```bash
./target/release/farsi_lang program.fr
```

---

## 📝 نمونه کد (زبان فارسی)

```farsi
// سلام جهان
نوشتن("سلام جهان!")

// متغیر
نام = "فارسی"
نسخه = 1.0

// تابع
تابع جمع(الف، ب) {
    برگردان الف + ب
}

// حلقه
برای (شمارنده = 1; شمارنده <= 10; شمارنده = شمارنده + 1) {
    نوشتن(شمارنده)
}

// شرط
اگر (سن > 18) {
    نوشتن("بزرگسال")
} وگرنه {
    نوشتن("نابالغ")
}
```

---

## 🚀 نقشه راه

### فاز 1: بنیان (تا ماه آینده)
- [ ] Lexer کامل
- [ ] Parser کامل
- [ ] Compiler ابتدایی
- [ ] Runtime پایه‌ای
- [ ] کتابخانه استاندارد ابتدایی

### فاز 2: توسعه (ماه‌های 2-3)
- [ ] پشتیبانی WebAssembly
- [ ] پشتیبانی شیء‌گرایی
- [ ] بهینه‌سازی‌های Compiler
- [ ] کتابخانه‌های وب

### فاز 3: تکمیل (ماه‌های 4-6)
- [ ] کتابخانه علم داده
- [ ] IDE Plugin
- [ ] Documentation کامل
- [ ] نسخه 1.0 رسمی

---

## 🤝 مشارکت

ما از مشارکت شما استقبال می‌کنیم! 🎉

```bash
# Fork کنید
# شاخه جدید ایجاد کنید
git checkout -b feature/نام-ویژگی

# کد خود را Commit کنید
git commit -m "افزودن ویژگی جدید"

# Push کنید
git push origin feature/نام-ویژگی

# Pull Request بسازید
```

---

## 📄 لایسنس

MIT License - برای جزئیات `LICENSE` فایل را ببینید

---

## 📞 تماس

- GitHub: https://github.com/hasibullahahmedi0-bot
- Issues: برای گزارش مشکلات

---

**ساخته‌شده با ❤️ برای جامعه برنامه‌نویسان فارسی**
