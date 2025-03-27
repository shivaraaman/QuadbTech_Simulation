mod blockchain;
use blockchain::chain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new(3);
    blockchain.add_block(vec!["Alice pays Bob 5 BTC".to_string()]);
    blockchain.add_block(vec!["Bob pays Charlie 2 BTC".to_string()]);
    for block in &blockchain.chain {
        println!("Index: {}", block.index);
        println!("Timestamp: {}", block.timestamp);
        println!("Transactions: {:?}", block.transactions);
        println!("Previous Hash: {}", block.previous_hash);
        println!("Hash: {}", block.hash);
        println!("Nonce: {}", block.nonce);
        println!("-----------------------------");
    }

    println!("Is blockchain valid? {}", blockchain.is_valid()); //it validate intergrity checks
}
