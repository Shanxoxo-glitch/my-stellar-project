# 🚀 Soroban Task Marketplace

A decentralized task marketplace built on the Stellar network using Soroban smart contracts.

---
<img width="1347" height="625" alt="Screenshot 2026-04-13 133805" src="https://github.com/user-attachments/assets/48f875d2-2a4c-41d9-a5f3-8a9551cad18a" />

## 🌍 Overview

Soroban Task Marketplace is a lightweight, on-chain platform that enables users to create, accept, and complete tasks without relying on centralized intermediaries.

It brings the concept of freelance micro-tasking to the blockchain — ensuring transparency, ownership, and secure coordination between participants.

---
DEPLOYED LINK
https://stellar.expert/explorer/testnet/contract/CD6AOLKD2KCYGC4D2TM4OXXG73MGMZ7EV3KRROUX6OHWWB3SIY2VQ2SL

## ✨ Key Highlights

* ⚡ Fully on-chain task management
* 🔐 Secure authorization via Stellar accounts
* 🧱 Built using Soroban smart contracts (Rust)
* 🧩 Modular and extensible design
* 🌐 Ready for dApp integration

---

## 🧠 How It Works

1. **Create Task**
   A user posts a task with a description and reward.

2. **Accept Task**
   Another user claims the task as a worker.

3. **Complete Task**
   The creator verifies and marks the task as completed.

4. **Track On-Chain**
   All task data is stored transparently on the blockchain.

---

## 🛠️ Features

### Core Functionality

* 📝 Create tasks with unique IDs
* 👷 Assign tasks to workers
* ✅ Mark tasks as completed
* 🔍 Retrieve task details anytime
* 🔐 Enforced authentication (`require_auth`)

---

### 🧱 Smart Contract Design

#### `Task` Structure

| Field         | Type    | Description       |
| ------------- | ------- | ----------------- |
| `id`          | u64     | Unique task ID    |
| `creator`     | Address | Task creator      |
| `description` | String  | Task details      |
| `reward`      | i128    | Reward amount     |
| `completed`   | bool    | Completion status |
| `worker`      | Option  | Assigned worker   |

---

### ⚙️ Contract Functions

| Function        | Description            |
| --------------- | ---------------------- |
| `create_task`   | Create a new task      |
| `accept_task`   | Assign worker to task  |
| `complete_task` | Mark task as completed |
| `get_task`      | Retrieve task details  |

---

## 🌐 Deployed Contract

🔗 **Contract Address:**
`CD6AOLKD2KCYGC4D2TM4OXXG73MGMZ7EV3KRROUX6OHWWB3SIY2VQ2SL`

👉 You can interact with it using the Soroban CLI or Stellar tools.

---

## 🧪 Example Usage

### Create a Task

```bash id="mkd82n"
soroban contract invoke \
  --id CD6AOLKD2KCYGC4D2TM4OXXG73MGMZ7EV3KRROUX6OHWWB3SIY2VQ2SL \
  --fn create_task \
  --arg creator=<YOUR_ADDRESS> \
  --arg description="Fix bug in frontend" \
  --arg reward=100
```

---

### Accept a Task

```bash id="8slx3k"
soroban contract invoke \
  --id CD6AOLKD2KCYGC4D2TM4OXXG73MGMZ7EV3KRROUX6OHWWB3SIY2VQ2SL \
  --fn accept_task \
  --arg worker=<YOUR_ADDRESS> \
  --arg task_id=1
```

---

### Complete a Task

```bash id="a9v2c1"
soroban contract invoke \
  --id CD6AOLKD2KCYGC4D2TM4OXXG73MGMZ7EV3KRROUX6OHWWB3SIY2VQ2SL \
  --fn complete_task \
  --arg creator=<YOUR_ADDRESS> \
  --arg task_id=1
```

---

## 🧰 Tech Stack

* **Smart Contract:** Rust + Soroban SDK
* **Blockchain:** Stellar (Soroban)
* **Storage:** On-chain key-value storage

---

## 🔐 Security Model

* ✅ Only creators can complete tasks
* ✅ Workers must explicitly accept tasks
* ✅ Prevents double assignment
* ✅ Prevents invalid task access

---

## ⚠️ Current Limitations

* ❌ No escrow/payment logic yet
* ❌ No frontend interface
* ❌ No task browsing index
* ❌ No dispute handling

---

## 🔮 Future Roadmap

* 💰 Add token escrow & automatic payouts
* 🌐 Build React frontend with wallet connect
* ⭐ Reputation system for workers
* 📊 Task indexing & filtering
* ⚖️ Dispute resolution mechanism
* 📣 Event logging for indexing

---

## 🏗️ Getting Started (Dev)

```bash id="k2pz8w"
# Build contract
soroban contract build

# Deploy contract
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/contract.wasm
```

---

## 🤝 Contributing

Pull requests are welcome!
Feel free to open issues for suggestions or improvements.

---

## 📄 License

MIT License

---

## 💡 Vision

To create a fully decentralized freelance economy where anyone can post, complete, and get rewarded for tasks — without intermediaries, fees, or restrictions.
