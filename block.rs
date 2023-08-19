use crate::queue::{Task, WorkQueue};
use digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::sync;

type Hash = GenericArray<u8, U32>;

#[derive(Debug, Clone)]
pub struct Block {
    prev_hash: Hash,
    generation: u64,
    difficulty: u8,
    data: String,
    proof: Option<u64>,
}

impl Block {
    // create and return a new initial block
    pub fn initial(difficulty: u8) -> Block {
        Block {
            prev_hash: Hash::default(),  // Initialize prev_hash to 32 zero bytes
            generation: 0,               // The genesis block has generation 0
            difficulty,                  // Difficulty is set as specified in the argument
            data: String::new(),         // Data is an empty string
            proof: None,                 // The genesis block has no proof initially
        }
    }
    
   // create and return a block that could follow `previous` in the chain
   pub fn next(previous: &Block, data: String) -> Block {
        Block {
            prev_hash: Self::hash(&previous), // what do we set the hash here
            generation: previous.generation + 1, // increment the generation by 1
            difficulty: previous.difficulty, // same difficulty as the previous block
            data, // data is provided as an argument
            proof: None, // no proof initially
        }
    }

    // create a string that we are going to hash later
    pub fn hash_string_for_proof(&self, proof: u64) -> String {
        let prev_hash_hex: String = self.prev_hash.iter().map(|b| format!("{:02x}", b)).collect();
        // construct the hash string
        let mut s = format!(
            "{}:{}:{}:{}:{}",
            prev_hash_hex,
            self.generation,
            self.difficulty,
            self.data,
            proof
        );

        s
    }

    pub fn hash_string(&self) -> String {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_string_for_proof(p)
    }

    pub fn hash_for_proof(&self, proof: u64) -> Hash {
        // construct the hash string
        let s = self.hash_string_for_proof(proof);

        // println!("{}",s);


        // compute the SHA256 hash of the string
        let mut hasher = Sha256::new();
        hasher.update(s.as_bytes());
        let result = hasher.finalize();

        result
    }

    pub fn hash(&self) -> Hash {
        // self.proof.unwrap() panics if block not mined
        let p = self.proof.unwrap();
        self.hash_for_proof(p)
    }

    pub fn set_proof(self: &mut Block, proof: u64) {
        self.proof = Some(proof);
    }


    // would this block be valid if we set the proof to `proof`?
    pub fn is_valid_for_proof(&self, proof: u64) -> bool {
        // calculate hash for given proof
        let hash = self.hash_for_proof(proof);

        // calculate the number of zero bytes and bits we want to check
        let n_bytes = self.difficulty / 8;
        let n_bits = self.difficulty % 8;

        // check the last n_bytes for zero values
        for i in 0..n_bytes {
            if hash[hash.len() - 1 - i as usize] != 0 {
                return false;
            }
        }

        // check the next byte for zero bits
        if n_bits > 0 {
            let byte = hash[hash.len() - 1 - n_bytes as usize];
            if byte % (1 << n_bits) != 0 {
                return false;
            }
        }
        true
    }


    pub fn is_valid(&self) -> bool {
        if self.proof.is_none() {
            return false;
        }
        self.is_valid_for_proof(self.proof.unwrap())
    }

    // Mine in a very simple way: check sequentially until a valid hash is found.
    // This doesn't *need* to be used in any way, but could be used to do some mining
    // before your .mine is complete. Results should be the same as .mine (but slower).
    pub fn mine_serial(self: &mut Block) {
        let mut p = 0u64;
        while !self.is_valid_for_proof(p) {
            p += 1;
        }
        self.proof = Some(p);
    }

    // this function splits a given range of proof-of-work values into roughly equal chunks, 
    // assigns these chunks to worker threads, 
    // and returns the first valid proof found, which makes the block's hash valid.
    // In block.mine_range(w, s, e, c):
    // start a WorkQueue with w workers
    // enqueue c MiningTasks, each of which checks an approximately-equal fraction of the range s to e.
    // wait for somebody to send a result, shut down the queue, and return the result.
    pub fn mine_range(self: &Block, workers: usize, start: u64, end: u64, mut chunks: u64) -> u64 {
        let mut queue = WorkQueue::new(workers);
        let block = sync::Arc::new(self.clone());
    
        // We should ensure the chunk_size is at least 1.
        if end < start {
            panic!("Invalid range: end is smaller than start!");
        }
    
        let range = end - start + 1;
        if chunks > range {
            chunks = range; // Reduce the chunks to the size of the range
        }
    
        let chunk_size = range / chunks;
        let mut remainder = range % chunks; // If the range isn't evenly divisible by chunks, we'll add the remainder to the first chunks
    
        for i in 0..chunks {
            let actual_chunk_size = if remainder > 0 {
                remainder -= 1;
                chunk_size + 1
            } else {
                chunk_size
            };
    
            let chunk_start = start + i * chunk_size;
            // No need for a condition here as we've already handled the case where end < start
            let chunk_end = chunk_start + actual_chunk_size - 1; 
            queue.enqueue(MiningTask::new(block.clone(), chunk_start, chunk_end)).unwrap();
        }
    
        // Receive and return the first valid proof
        for proof in queue.iter() {
            return proof;
        }
    
        panic!("No valid proof found in the given range!");
    }

    // decides the range of values that we are going to mine with worker threads
    pub fn mine_for_proof(self: &Block, workers: usize) -> u64 {
        let range_start: u64 = 0;
        let range_end: u64 = 8 * (1 << self.difficulty); // 8 * 2^(bits that must be zero)
        let chunks: u64 = 2345;
        self.mine_range(workers, range_start, range_end, chunks)
    }

    pub fn mine(self: &mut Block, workers: usize) {
        self.proof = Some(self.mine_for_proof(workers));
    }
}

// MiningTask is a struct that represents a mining task in the blockchain context.
// It contains the block to be mined, and the start and end values representing 
// the range of proof-of-work values that will be checked for this task.
pub struct MiningTask {
    block: sync::Arc<Block>,
    start: u64,
    end: u64,
}

// It takes a reference-counted block and the start and end of a range 
// to create a new MiningTask instance.
impl MiningTask {
    pub fn new(block: sync::Arc<Block>, start: u64, end: u64) -> MiningTask {
        MiningTask {
            block,
            start,
            end,
        }
    }
}

// The Output type for this task is a u64, representing a valid proof-of-work.
impl Task for MiningTask {
    type Output = u64;

    // The run function iterates through the assigned range of proofs.
    // If it finds a valid proof, it returns it immediately.
    // each worker thread runs this in parralel
    fn run(&self) -> Option<u64> {
        for proof in self.start..=self.end {
            if self.block.is_valid_for_proof(proof) {
                return Some(proof);
            }
        }
        None
    }
}
