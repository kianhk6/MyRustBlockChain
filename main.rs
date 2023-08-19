
use a3::block::{Block, MiningTask};

fn main() {
    let mut b0 = Block::initial(8);
    b0.mine(4);
    println!("{}", b0.hash_string());
    println!("{:02x}", b0.hash());
    let mut b1 = Block::next(&b0, String::from("hello I am string 1"));
    b1.mine(1);

    let mut prevHash = b0.hash();
        
    let mut result = String::new();

    for byte in prevHash.iter() {
        write!(&mut result, "{:02x}", byte).expect("Failed to write to string"); // Convert each byte to hexadecimal and append to the result
    }

    result = result + ":1:0:hello I am string 1:0";
    println!("{}", result);

    println!("{}", b1.hash_string());


    // println!("{:02x}", b1.hash());
    // let mut b2 = Block::next(&b1, String::from("this is not interesting"));
    // b2.mine(4);
    // println!("{}", b2.hash_string());
    // println!("{:02x}", b2.hash());


    // let mut b1 = Block::next(&b0, String::from("message"));
    // b1.set_proof(2159);
    

    // when we set the proof do we re evaluate the hash? 


    
    // let prev_hash_hex: String = b0.prev_hash.iter().map(|b| format!("{:02x}", b)).collect();
    // println!("previous of {}", prev_hash_hex);
    // // construct the hash string
    // let mut s = format!(
    //     "{}:{}:{}:{}:{}",
    //     prev_hash_hex,
    //     b0.generation,
    //     b0.difficulty,
    //     b0.data,
    //     b0.proof.unwrap()
    // );
    
    // println!("{}",s);

    // // // compute the SHA256 hash of the string
    // let mut hasher = Sha256::new();
    // hasher.update(s.as_bytes());
    // let result = hasher.finalize();

    // // // convert the hash to a hex string and return it
    // s.clear();
    // write!(&mut s, "{:02x}", result).unwrap();




    // let mut b1 = Block::next(&b0, String::from("hash example 1234"));
    // b1.set_proof(1407891);

    // now this needs to be previous hash of b1

    // Nothing is required here, but it may be useful for testing.
}

