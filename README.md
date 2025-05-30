
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

## Getting Started
### Step 1: Pull the latest Docker image
```bash
docker pull saurabhwadekar/castify:latest
```
### Step 2: Run the Castify server with environment variables
Run the server by specifying required environment variables:
```bash
docker run -d -p 8000:8000 \
    -e SERVER_SECRET=your_secret_key_here \
    -e VERIFICATION_URL=http://localhost:3000/api/verify \
    -e GLOBAL_TOKEN=your_global_token_here \
    -e USE_GLOBAL_TOKEN=true \
    saurabhwadekar/castify:latest
```
| Environment Variable | Description                                                                                    |
| -------------------- | ---------------------------------------------------------------------------------------------- |
| `SERVER_SECRET`      | A secret token used for securing communication between your backend and Castify                |
| `VERIFICATION_URL`   | Backend API URL to verify user tokens (used only if `USE_GLOBAL_TOKEN=false`)                  |
| `GLOBAL_TOKEN`       | Shared token used by all clients if global token mode is enabled (`USE_GLOBAL_TOKEN=true`)     |
| `USE_GLOBAL_TOKEN`   | Set to `true` to use global token authentication; `false` for user-specific token verification |

### Server Health Check
To verify the server is running correctly, send a GET request to the root endpoint:
```bash
curl http://localhost:8000
```
Expected response:
```bash
running
```
WebSocket Client Connection
If using Global Token Authentication (`USE_GLOBAL_TOKEN=true`)
Clients must connect to the WebSocket server with the global token appended as a query parameter:
```bash
ws://127.0.0.1:8000/ws?token=your_global_token_here
```
If using User-based Token Authentication (USE_GLOBAL_TOKEN=false)
- Your backend must provide an API endpoint (VERIFICATION_URL) to validate user tokens.
- When a client connects with their token:
```bash
ws://127.0.0.1:8000/ws?token=user_token_here
```
- Castify sends this token to your backend verification API.
- If your backend responds with HTTP status 200 OK, the connection is accepted. Otherwise, it is rejected.

### Secure Backend Communication
- Castify uses the SERVER_SECRET to secure communication with your backend verification API.
- It sends the SERVER_SECRET as an authorization token in the request headers to VERIFICATION_URL.
- Your backend should verify this secret token to ensure requests are coming from the trusted Castify server.

### Broadcasting Messages to Clients
Castify provides an HTTP POST API to broadcast custom JSON messages to all connected WebSocket clients.
#### Endpoint
```bash
POST http://localhost:8000/broadcast
```
#### Request Body Format
```json
{
  "message": {
    "event": "update",
    "details": {
      "items": [1, 2, 3, 4],
      "status": "completed",
      "meta": {
        "source": "system",
        "attempt": 2
      }
    }
  },
  "token": "your_secret_key_here"
}
```
- `message` can be any custom JSON data you want to send to clients.
- `token` must match the `SERVER_SECRET` set in the server environment variables to authorize the broadcast request.

### Example using `curl`
```bash
curl -X POST http://localhost:8000/broadcast \
  -H "Content-Type: application/json" \
  -d '{
    "message": {
      "event": "update",
      "details": {
        "items": [1, 2, 3, 4],
        "status": "completed",
        "meta": {
          "source": "system",
          "attempt": 2
        }
      }
    },
    "token": "your_secret_key_here"
  }'
```
### Summary
1. Pull the Docker image.

2. Run the Castify server with required environment variables.

3. Connect clients to WebSocket endpoint with appropriate token.

4. Verify tokens either via global token or backend API.

5. Broadcast JSON messages securely using the `/broadcast` HTTP endpoint with the `SERVER_SECRET`.

### Optional: Backend Token Verification API Example
Your backend should implement an API that:

- Accepts a token parameter from Castify.

- Checks the token validity (e.g., against your user database).

- Verifies that the request contains the correct `SERVER_SECRET` in headers.

- Returns HTTP 200 OK if valid; otherwise, returns 401 Unauthorized.

This API URL should be set as `VERIFICATION_URL` in Castify’s environment variables.








