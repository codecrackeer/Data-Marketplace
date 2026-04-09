# 📊 Data Marketplace DApp (Soroban - Stellar)

<img width="1919" height="968" alt="image" src="https://github.com/user-attachments/assets/7dc28900-1dd4-41d4-b1bd-a6765c6721fb" />


## 🚀 Project Description

The **Data Marketplace DApp** is a decentralized application built using **Soroban smart contracts on the Stellar blockchain**. It enables users to list, discover, and purchase datasets in a secure, transparent, and trustless environment.

This project showcases how blockchain technology can eliminate intermediaries and provide a decentralized infrastructure for data exchange and monetization.

---

## ⚙️ What it does

* Allows users to list datasets with a name and price
* Stores dataset metadata securely on-chain
* Enables buyers to purchase datasets
* Records transactions immutably on the blockchain
* Ensures only authorized users can perform actions

---

## ✨ Features

* 📦 **Data Listing** – Add datasets with pricing
* 🔐 **Ownership Verification** – Only owners can add their data
* 💰 **Purchase Functionality** – Buyers can initiate dataset purchases
* 📜 **On-chain Storage** – Dataset metadata stored in smart contract
* 📊 **Event Logging** – Tracks purchase activity for transparency

---

## 🛠️ Tech Stack

* **Soroban** – Smart contract platform on Stellar
* **Rust** – Contract development language
* **Stellar Blockchain (Testnet)**

---

## 🔗 Deployed Smart Contract Link

👉 https://stellar.expert/explorer/testnet/contract/CCMSQ324XX4HGYFGLRGMTXTG3A6NUSLKWDYQOQT7SPDT3MSX6LKY2I6M

---

## 🧪 How to Build & Deploy

### 1️⃣ Build Contract

```bash
stellar contract build
```

### 2️⃣ Deploy (Testnet)

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source <your-identity> \
  --network testnet
```

---

## 📌 Future Improvements

* 🔗 Token-based payments (XLM / custom token)
* ☁️ IPFS integration for dataset storage
* 🔐 Access control after purchase
* ⭐ Rating & review system
* 🌐 Frontend UI (React + Stellar SDK)

---

## 👨‍💻 Author

**Srija Mondal**
B.Tech CSE (Data Science)

---

## 📜 License

This project is open-source and available under the MIT License.
