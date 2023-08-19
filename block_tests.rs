#[cfg(test)]

mod block_tests {
    use crate::block::{Block, MiningTask};
    use std::fmt::Write;

    #[test]
    fn test_mine_block_difficulty_0() {
        let mut b0 = Block::initial(0);
        b0.mine(1);
        assert!(b0.hash_string() == "0000000000000000000000000000000000000000000000000000000000000000:0:0::0");
        assert!(b0.is_valid());

        let mut b1 = Block::next(&b0, String::from("hello I am string 1"));
        b1.mine(1);

        let mut prevHash = b0.hash();
        
        let mut result = String::new();

        for byte in prevHash.iter() {
            write!(&mut result, "{:02x}", byte).expect("Failed to write to string"); // Convert each byte to hexadecimal and append to the result
        }

        result = result + ":1:0:hello I am string 1:0";
        assert!(b1.hash_string() == result);
        assert!(b1.is_valid());

    }

    #[test]
    fn test_mine_block_difficulty_24() {
        // as proof is a private field I need to test it with an constant 
        let mut b0 = Block::initial(24);
        b0.mine(10);
        assert!(b0.hash_string() == "0000000000000000000000000000000000000000000000000000000000000000:0:24::6087348");
        assert!(b0.is_valid());

        let mut b1 = Block::next(&b0, String::from("hello I am string 1"));
        b1.mine(10);

        let mut prevHash = b0.hash();
        
        let mut result = String::new();

        for byte in prevHash.iter() {
            write!(&mut result, "{:02x}", byte).expect("Failed to write to string"); // Convert each byte to hexadecimal and append to the result
        }

        result = result + ":1:24:hello I am string 1:2149613";
        assert!(b1.hash_string() == result);
        assert!(b1.is_valid());

    }

    #[test]
    fn test_mine_block_difficulty_valid_Invalid() {
        // as proof is a private field I need to test it with an constant 
        let mut b0 = Block::initial(24);
        b0.mine(10);
        assert!(b0.hash_string() == "0000000000000000000000000000000000000000000000000000000000000000:0:24::6087348");
        assert!(b0.is_valid());

        let mut b1 = Block::next(&b0, String::from("hello I am string 1"));
        b1.set_proof(0);

        let mut prevHash = b0.hash();
        
        let mut result = String::new();

        for byte in prevHash.iter() {
            write!(&mut result, "{:02x}", byte).expect("Failed to write to string"); // Convert each byte to hexadecimal and append to the result
        }

        result = result + ":1:24:hello I am string 1:0";
        assert!(b1.hash_string() == result);
        assert!(!b1.is_valid());
    }
    
    #[test]
    fn test_mine_block_difficulty_valid_1_miner_then_invalid() {
        // as proof is a private field I need to test it with an constant 
        let mut b0 = Block::initial(8);
        b0.mine(1);
        assert!(b0.hash_string() == "0000000000000000000000000000000000000000000000000000000000000000:0:8::529");
        assert!(b0.is_valid());

        let mut b1 = Block::next(&b0, String::from("hello I am string 1"));
        b1.set_proof(0);
        assert!(!b1.is_valid());
    }

}

