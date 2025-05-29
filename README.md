
![ChatGPT Image May 26, 2025, 02_05_19 PM](https://github.com/user-attachments/assets/1ea1fb22-6199-479d-aac9-ecf86c835461)
# Castify
**Castify** — a blazing-fast, Rust-powered real-time broadcast server 🚀

![License](https://img.shields.io/github/license/YOUR_USERNAME/castify)
![Rust](https://img.shields.io/badge/Rust-🦀-orange)
![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen)

---

## ✨ What is Castify?

Castify is a **high-performance, one-way real-time broadcasting server** built in Rust.  
It allows backend systems to **send messages to thousands of connected clients simultaneously**.

It’s perfect for:
- 🔔 **Live notifications** (news, alerts, updates)
- 📈 **Dashboards** (real-time data feeds)
- 📡 **Stock tickers, price feeds**
- 🛠 **Server event broadcasting**

⚠ **Note:**  
Castify is not designed for chat apps or multiplayer games, as it only supports **server → client** messages (no client-to-client or client-to-server communication).

---

## 🚀 Features

✅ Rust-powered speed and efficiency  
✅ One-way real-time push (server → clients)  
✅ Easy backend integration via API  
✅ Secure per-connection authentication  
✅ Scales to thousands of concurrent clients  
✅ Minimal resource usage (runs smoothly on mid-tier hardware)

---

## 🏗 Architecture Overview


- **Backend API** → Sends messages to Castify  
- **Castify** → Authenticates clients, broadcasts messages  
- **Clients** → Connect over WebSocket, receive real-time updates

---

## ⚙ Installation

```bash
git clone https://github.com/saurabhwadekar/castify.git
cd castify
cargo build --release
```
## ⚙ Docker
```bash
docker pull saurabhwadekar/castify:latest
```
```bash
docker run -d -p 8000:8000 \
    -e SERVER_SECRET=your_secret_key_here \
    -e VERIFICATION_URL=http://localhost:3000/api/verify \
    -e GLOBAL_TOKEN=your_global_token_here \
    -e USE_GLOBAL_TOKEN=true \
    saurabhwadekar/castify:latest
```

