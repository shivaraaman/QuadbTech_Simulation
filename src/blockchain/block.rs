use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<String>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    // Function to create a new block
    pub fn new(index: u64, transactions: Vec<String>, previous_hash: String, difficulty: usize) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        };
        block.mine_block(difficulty); // Perform mining
        block
    }

    // Function to calculate hash
    pub fn calculate_hash(&self) -> String {
        let data = format!(
            "{}{}{:?}{}{}",
            self.index, self.timestamp, self.transactions, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
    

    // Proof-of-Work: Mine the block
    pub fn mine_block(&mut self, difficulty: usize) {
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(&"0".repeat(difficulty)) {
                break;
            }
            self.nonce += 1;
        }
    }
}
