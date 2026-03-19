# 📜 Proof of Existence – Soroban Smart Contract
<img width="942" height="463" alt="Screenshot 2026-03-19 133726" src="https://github.com/user-attachments/assets/094696fc-4e93-4365-874c-543e7a112c9c" />

## 🚀 Project Description

This project implements a **Proof of Existence** smart contract on the Stellar Soroban platform.  
It allows users to store a cryptographic hash of any document or data on-chain, proving that the data existed at a specific point in time without revealing the actual content.

This is useful for verifying ownership, authenticity, and integrity of digital assets.

---

## 🔍 What It Does

- Users submit a **hash (SHA-256 or similar)** of a document.
- The smart contract stores the hash along with a **ledger timestamp**.
- Anyone can later verify:
  - Whether the document existed
  - When it was recorded

No actual document data is stored — ensuring **privacy and security**.

---

## ✨ Features

- ✅ Store proof (hash of data)
- ✅ Verify existence of proof
- ✅ Retrieve timestamp of stored proof
- ✅ Fully decentralized and tamper-proof
- ✅ Low-cost and efficient (Soroban-based)

---

## ⚙️ How It Works

1. Convert your document into a hash (e.g., SHA-256)
2. Call `store_proof(hash)`
3. The contract stores:
   - Hash → Timestamp

Later:
- Use `verify_proof(hash)` → returns `true/false`
- Use `get_proof(hash)` → returns timestamp

---

## 📦 Functions

| Function         | Description                         |
|----------------|-------------------------------------|
| store_proof     | Stores hash with timestamp          |
| verify_proof    | Checks if proof exists              |
| get_proof       | Retrieves stored timestamp          |

---

## 🔗 Deployed Smart Contract Link
https://lab.stellar.org/r/testnet/contract/CDRIZ4CBGE7QPVTQ7X4M3K36YCFHPO6BYQAHVNALZJMKBXWEEQ25FVBX

---

## 🛠️ Tech Stack

- Rust
- Soroban SDK
- Stellar Blockchain

---

## 📌 Use Cases

- Document verification
- Intellectual property protection
- Legal timestamping
- Academic certification
- Digital asset ownership proof

---

## ⚠️ Notes

- Only hashes should be stored, not raw data.
- Ensure you use a secure hashing algorithm (SHA-256 recommended).

---

## 👨‍💻 Author

Built as a Soroban project for learning and real-world blockchain use cases.
