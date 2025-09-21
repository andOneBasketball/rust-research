enum BlockChain {
    BitCoin,
    Ethereum,
    Starknet,
    Solana,
}

fn main() {
    let block_chain = BlockChain::Solana;
    match block_chain {
        BlockChain::BitCoin => println!("BitCoin"),
        // X | Y，类似逻辑运算符 或，代表该分支可以匹配 X 也可以匹配 Y，只要满足一个即可
        BlockChain::Ethereum | BlockChain::Starknet => {
            println!("Ethereum or Starknet");
        },
        // 使用 _ 来代表未列出的所有可能性
        _ => println!("Solana"),
    };
}