# 🦀 Cryptopals Challenges - Rust

This repository contains my solutions to the [Cryptopals Crypto Challenges](https://www.cryptopals.com/), implemented in **Rust**.
The primary goal is to master the language's safety and performance while learning how to break and build cryptographic systems.

---

## Project Structure

The project is organized as a **Cargo Workspace** to ensure modularity and code reuse:

* **`crypto-lib/`**: A shared library containing reusable cryptographic utilities (XOR operations, scoring functions, etc.).
* **`set1/, set2/, set3/, ...`**: Solutions for each set of challenges .

--- 
## Usage

### Prerequisites
Ensure you have the [Rust toolchain](https://rustup.rs/) installed.

### Clone the repository

```bash
git clone https://github.com/Misak0o/crypto-challenges.git
cd crypto-challenges    
```

### Run Tests
The challenges are validated using Rust's built-in testing framework. To run all tests across the workspace:
```bash
cargo test
```

---

## Disclaimer
These implementations are for **educational purposes only**. Do not use this code to secure real-world data. As the saying goes: *"Don't roll your own crypto."*

---

## License

This project is licensed under the **MIT License**.

Copyright (c) 2026 Misak0o.
