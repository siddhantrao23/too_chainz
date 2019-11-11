use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 0, 
    "Genisis block!".to_owned(), 0x0000ffffffffffffffffffffffffffff);
    println!("{:?}", &block);
    
    block.mine();
    println!("{:?}", &block);
    
    block.mine();
    println!("{:?}", &block);
}
