# Simple Blockchain in Rust

## Introduction
This project is a simple implementation of a **blockchain** using the Rust programming language. It includes **Proof of Work (PoW)**, mining, and block validation, demonstrating core blockchain concepts.

## Features
- ✅ **Blockchain with Proof of Work (PoW)**
- ✅ **Blocks with Index, Timestamp, Transactions, Hash, and Nonce**
- ✅ **Mining Algorithm to Solve Hash Puzzle**
- ✅ **Dynamic Block Creation & Validation**

## Project Structure
```
QuadbTech/
├── src/
│   ├── main.rs          # Entry point
│   ├── blockchain/
│   │   ├── mod.rs       # Module definition
│   │   ├── block.rs     # Block structure and PoW logic
│   │   ├── chain.rs     # Blockchain structure and validation
│   ├── utils/
│   │   ├── hash.rs      # Hashing function
│   │   ├── difficulty.rs # Difficulty adjustment (if needed)
├── Cargo.toml           # Dependencies and metadata
├── README.md            # Project documentation
```

## Dependencies
This project uses the following Rust crates:
```toml
[dependencies]
sha2 = "0.10"      # For hashing blocks (SHA-256)
serde = { version = "1.0", features = ["derive"] } # For serialization
serde_json = "1.0" # For JSON encoding
```

## How It Works
### 1. Creating a Block
Each block contains:
- **Index**: Position in the blockchain.
- **Timestamp**: Time of block creation.
- **Transactions**: List of transactions.
- **Previous Hash**: Hash of the previous block.
- **Nonce**: Used for Proof of Work.
- **Hash**: Unique identifier generated using SHA-256.

### 2. Mining a Block (Proof of Work)
Blocks must satisfy a **difficulty condition** (leading zeros in the hash). This is achieved by iterating over `nonce` values until a valid hash is found:
```rust
fn mine_block(&mut self, difficulty: usize) {
    loop {
        self.hash = self.calculate_hash();
        if self.hash.starts_with(&"0".repeat(difficulty)) {
            break;
        }
        self.nonce += 1;
    }
}
```

### 3. Adding a Block
A new block is added only if it passes validation (i.e., hash matches difficulty and links correctly to the previous block).

### 4. Validating the Blockchain
The `is_valid()` method ensures:
- Each block’s hash is correctly calculated.
- Each block links properly to the previous block.

## Running the Project
1️⃣ **Clone the repository**
```sh
git clone https://github.com/yourusername/quadbtech-blockchain.git
cd quadbtech-blockchain
```

2️⃣ **Build the project**
```sh
cargo build
```

3️⃣ **Run the blockchain**
```sh
cargo run
```

## Future Enhancements 🚀
- ⏳ **Dynamic Difficulty Adjustment**
- 🛠 **Network P2P Integration**
- 💰 **Implement Transactions with Signatures**
- 🔄 **Switch to Proof of Stake (PoS) Experimentally**

## Conclusion
This project provides a **fundamental understanding** of blockchain technology, mining, and PoW. It's a great starting point for learning about decentralized systems in Rust!

---
📌 **Author:** Sivaraaman | 🌐 **Project for QuadBTech Internship**

