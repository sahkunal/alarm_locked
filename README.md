# ⏰ Alarm Locked — Time-Locked Vault on Solana

### 🔐 Deterministic On-Chain Security | Anchor • PDA Architecture • Next.js dApp

<p align="center">
  <strong>A programmable vault where time — not trust — controls access.</strong>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Solana-Devnet-purple?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Anchor-Rust-blue?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Next.js-Frontend-black?style=for-the-badge" />
  <img src="https://img.shields.io/badge/Status-Capstone%20Complete-success?style=for-the-badge" />
</p>

---

# ✨ Overview

**Alarm Locked** is a time-locked vault built using the Anchor framework on Solana.
Users deposit SOL into a PDA-controlled vault that **cannot be withdrawn until a predefined unlock timestamp**.

Instead of relying on off-chain timers or user promises, the contract enforces rules directly using Solana’s `Clock` sysvar.

> 💡 The blockchain becomes the alarm clock.

---

# 🧠 Unique Constraint — Time Lock Enforcement

This project implements a **time-based restriction**, one of the core vault constraint patterns.

### Core Rule

```
Withdrawal allowed ONLY when:
current_time >= unlock_time
```

### Why This Matters

Traditional apps trust backend logic.
This vault removes trust entirely:

* No backend timers
* No admin overrides
* No manual approvals

Only deterministic on-chain time.

---

# 🔧 How It Works

## 🏗️ PDA Architecture

| Account       | Seeds                     | Role                          |
| ------------- | ------------------------- | ----------------------------- |
| `vault_state` | `[b"state", owner]`       | Stores metadata & unlock time |
| `vault`       | `[b"vault", vault_state]` | Holds locked SOL              |

The vault PDA signs transactions using program seeds — meaning **users cannot bypass rules**.

---

## ⚙️ Instruction Flow

### 1️⃣ Initialize Vault

Creates state PDA and defines unlock timestamp.

### 2️⃣ Deposit

Transfers SOL into vault PDA before unlock.

### 3️⃣ Withdraw

Allowed only after unlock time passes.

### 4️⃣ Close Vault

Closes PDA after funds are withdrawn.

---

# 🔐 Constraint Visualization

```
User Deposit
     │
     ▼
┌───────────────┐
│   Vault PDA   │
└───────────────┘
        │
        │  Clock Sysvar Check
        ▼
 IF current_time < unlock_time
        ❌ REJECT
 ELSE
        ✅ ALLOW WITHDRAW
```

---

# 🌐 Frontend — Professional dApp UI

A modern Next.js interface powers the interaction layer.

### Features

* Wallet Connect (Phantom)
* Glassmorphism animated UI
* Live vault balance display
* Unlock countdown timer
* Toast notifications for transactions
* Framer Motion animations

Run locally:

```bash
cd app
npm install
npm run dev
```

Open:

```
http://localhost:3000
```

---

# 🧪 Testing

Complete Anchor test suite validates:

* Initialization logic
* Deposit transfers
* Locked withdrawal rejection
* Successful withdrawal post unlock
* Vault closure

Run tests:

```bash
anchor test
```

---

# 🚀 Devnet Deployment

**Program ID**

```
8SKpWVeyrbDTJpGztuEVK399jHSx5n2HuAGSAjgHKGQo
```

Explorer:

https://explorer.solana.com/address/8SKpWVeyrbDTJpGztuEVK399jHSx5n2HuAGSAjgHKGQo?cluster=devnet

---

# 🛠 Tech Stack

* 🦀 Anchor (Rust)
* ⚡ Solana Web3.js
* 🧩 PDA Account Model
* 🌐 Next.js App Router
* 🎨 TailwindCSS
* ✨ Framer Motion
* 🔔 React Hot Toast

---

# 📁 Project Structure

```
alarm_locked/
 ├── programs/          → Anchor smart contract
 ├── tests/             → TypeScript tests
 ├── app/               → Next.js frontend
 │    ├── app/page.tsx
 │    ├── providers.tsx
 │    └── lib/anchor.ts
 └── Anchor.toml
```

---

# 🧩 Design Philosophy

Alarm Locked demonstrates a key Web3 principle:

> Replace human trust with deterministic rules.

Instead of asking:

> “Will the user wait?”

The contract guarantees:

> “The user must wait.”

---

# 🎯 Capstone Requirements Checklist

✔ Anchor Framework
✔ Unique Constraint (Time Lock)
✔ PDA Usage
✔ Automated Tests
✔ Devnet Deployment
✔ Documentation
✔ Frontend Integration

---

# 📸 Screenshots

Add UI screenshots here:

```
Screenshot 2026-02-22 204246.png
Screenshot 2026-02-22 204257.png
```

---

# 🧑‍💻 Author

**Kunal Sah**
Solana Builder • Smart Contract Developer

---

<p align="center">
  Built with ⚡ on Solana Devnet
</p>
