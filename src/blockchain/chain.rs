use super::block::Block;

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    // Create a new blockchain with a genesis block
    pub fn new(difficulty: usize) -> Self {
        let genesis_block = Block::new(0, vec!["Genesis Block".to_string()], "0".to_string(), difficulty);
        Blockchain {
            chain: vec![genesis_block],
            difficulty,
        }
    }

    // Add a new block to the blockchain
    pub fn add_block(&mut self, transactions: Vec<String>) {
        let previous_hash = self.chain.last().unwrap().hash.clone();
        let new_block = Block::new(self.chain.len() as u64, transactions, previous_hash, self.difficulty);
        self.chain.push(new_block);
    }

    // Validate the blockchain
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];

            if current.hash != current.calculate_hash() || current.previous_hash != previous.hash {
                return false;
            }
        }
        true
    }
}
