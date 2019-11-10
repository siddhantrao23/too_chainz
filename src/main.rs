use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 0, "Genisis block!".to_owned());
    println!("{:?}", &block);

    block.hash = block.hash();
    println!("{:?}", &block);
}
