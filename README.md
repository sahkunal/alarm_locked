# ⏰ Alarm Locked – Time-Locked Vault (Anchor Capstone)

## 📌 Overview

**Alarm Locked** is a Solana smart contract built using the **Anchor framework** that implements a **Time-Locked Vault**.

The vault allows a user to deposit SOL into a Program Derived Address (PDA) and prevents withdrawals until a predefined `unlock_time` is reached. This demonstrates secure custody, PDA authorization, and time-based constraints on Solana.

---

## 🎯 Objective

This project was created as a capstone task to design and implement a vault with a **unique constraint** using Anchor.

The chosen design is a **Time-Locked Vault**, where:

* Funds can be deposited before the unlock time.
* Withdrawals are strictly blocked until the unlock timestamp.
* Only the vault owner can withdraw funds.

---

## 🔐 Unique Constraint – Time Lock Logic

The core security rule of this vault is a **time-based restriction**:

* During initialization, an `unlock_time` is set.
* The program reads the Solana **Clock sysvar** to compare current time with the unlock time.
* Withdrawals fail if:

```
current_time < unlock_time
```

This ensures funds remain locked until the defined moment.

---

## 🧱 Architecture

### Program Derived Addresses (PDAs)

The vault uses two PDAs:

1. **Vault State PDA**

   * Seeds: `["state", owner_pubkey]`
   * Stores:

     * owner
     * unlock_time
     * bump seeds
     * initialization flag

2. **Vault PDA**

   * Seeds: `["vault", vault_state_pubkey]`
   * Holds deposited SOL

Using PDAs ensures that only the program can authorize transfers from the vault.

---

## ⚙️ Instructions

### 1. Initialize

Creates the vault state and sets the unlock timestamp.

```
initialize(unlock_time)
```

Rules:

* Unlock time must be in the future.

---

### 2. Deposit

Transfers SOL from the owner into the vault PDA.

```
deposit(amount)
```

Rules:

* Only owner can deposit.
* Deposits allowed only before unlock time.

---

### 3. Withdraw

Transfers all SOL from the vault PDA back to the owner.

```
withdraw()
```

Rules:

* Only owner can withdraw.
* Allowed only after unlock time.

---

### 4. Close Vault

Closes the vault state account once funds are withdrawn.

```
close_vault()
```

Rules:

* Vault must be empty.
* Only owner can close.

---

## 🧪 Tests

The test suite demonstrates:

* ✅ Vault initialization
* ✅ SOL deposit
* ✅ Withdrawal blocked before unlock time
* ✅ Successful withdrawal after unlock time
* ✅ Vault closure

Run tests locally:

```bash
yarn install
anchor test
```

---

## 🚀 Deployment

Network: **Devnet**

Program ID:

```
8SKpWVeyrbDTJpGztuEVK399jHSx5n2HuAGSAjgHKGQo
```

To deploy:

```bash
solana config set --url devnet
anchor build
anchor deploy
```

---

## 🛠 Tech Stack

* Rust
* Anchor Framework
* Solana Web3.js
* TypeScript
* Mocha / Chai

---

## 📂 Project Structure

```
programs/alarm_locked/src/lib.rs   → Smart contract logic
tests/alarm_locked.ts              → Test suite
Anchor.toml                        → Anchor configuration
```

---

## 🔎 Security Notes

* PDA signer seeds prevent unauthorized withdrawals.
* Clock sysvar enforces immutable time-based rules.
* Custom error handling improves clarity and safety.

---

## 📸 Test Results

(Add a screenshot of your passing `anchor test` output here.)

---

## 👤 Author

Alarm Locked – Time Locked Vault
Built using Anchor for Solana Capstone Submission.
