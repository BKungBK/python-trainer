# Python Trainer By BKungBK

แอปพลิเคชันเดสก์ท็อปสำหรับการฝึกฝนเขียนโค้ดภาษา Python ประจำวัน พร้อมการติดตามเป้าหมายการเรียนรู้และซิงค์ข้อมูลผ่านระบบคลาวด์ พัฒนาด้วยสถาปัตยกรรมประสิทธิภาพสูง

## 🛠️ เทคโนโลยีที่ใช้ (Tech Stack)
*   **Frontend:** SvelteKit + Svelte 5 + Monaco Editor
*   **Backend:** Tauri v2 (Rust)
*   **Database:** SQLite (Local) + Supabase (Cloud Sync)

---

## 🚀 เริ่มต้นใช้งานสำหรับผู้พัฒนา (Getting Started)

### 1. ตั้งค่าสภาพแวดล้อม (Prerequisites)
ตรวจสอบให้แน่ใจว่าเครื่องของคุณมี:
*   [Node.js](https://nodejs.org/) (เวอร์ชันล่าสุด)
*   [Rust](https://www.rust-lang.org/)
*   [Python 3](https://www.python.org/) (สำหรับรันรหัสเพื่อตรวจผลลัพธ์)

### 2. ตั้งค่าไฟล์ความลับ (.env)
คัดลอกไฟล์ `.env.example` เป็น `.env` และกรอกข้อมูลการเชื่อมต่อ Supabase ของคุณ:
```bash
cp .env.example .env
```

### 3. รันคำสั่งเริ่มพัฒนา
ติดตั้งโปรแกรมและสั่งเปิดโหมดพัฒนา (Development Mode):
```bash
npm install
npm run dev
```

### 4. การส่งออกแอปพลิเคชัน (Build / Export)
เพื่อส่งออกตัวติดตั้งแอปพลิเคชันสำหรับใช้งานจริง (เช่น `.msi` ของ Windows):
```bash
# บน Windows PowerShell (สำหรับลงชื่อตรวจเช็กความปลอดภัยของ Auto-update)
$env:TAURI_SIGNING_PRIVATE_KEY="src-tauri/tauri-key.key"

# สั่ง Build ตัวโปรแกรม
npm run tauri build
```
ไฟล์ติดตั้งที่ได้จะอยู่ในไดเรกทอรี `src-tauri/target/release/bundle/`
