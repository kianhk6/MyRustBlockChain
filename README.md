# MyRustBlockChain

## Introduction
This project involves the creation of a simple blockchain system, incorporating proof-of-work which requires computational effort for validating blocks. A code skeleton with structure and hints is provided as a starting point.

### Blockchain Basics
The blockchain is composed of blocks, each containing data hashed using a cryptographic function. The validity of each block's hash depends on its contents and its alignment with proof-of-work criteria. Blocks are linked in a chain, with each block containing the hash value of its predecessor. This structure allows for the entire chain to be verified with minimal information.

### Proof-Of-Work Basics
Proof-of-work necessitates computational effort to generate a valid block. The proof value in each block is an integer, and the block is considered valid if its hash ends with a specified number of zero bits. This process ensures that block generation requires effort, yet the verification of such blocks remains simple.

### Work Queue
The system utilizes a parallel computation approach for proof-of-work calculations. It features a work queue architecture where tasks are distributed among several worker threads and the results are communicated via single-producer multiple-consumer and multiple-producer single-consumer channels. The work queue allows for efficient computation and early termination of unnecessary calculations once a valid proof is found.

### Stopping the Queue
The queue system is designed to halt once a valid proof-of-work is found. This involves shutting down the task channel, allowing worker threads to exit, and handling remaining tasks and thread closures properly.

### Blocks
The project includes functionality to create blockchain blocks. Each block holds specific data, and there is a special case for the first block in the chain. Functions are provided to generate the initial block and subsequent blocks in the chain.

### Valid Blocks
A block is valid if its proof-of-work value results in a hash ending in the specified number of zero bits. Functions are provided to calculate the hash for a given proof and to check the validity of the hash.

### Mining
Mining involves finding a valid proof-of-work for a block. This is achieved by iterating over potential proof values. The project includes a simplified serial mining function and a more complex parallel approach using the work queue.

### Mining Tasks
Mining tasks are structured to fit within the work queue system. These tasks are distributed among worker threads to efficiently find a valid proof-of-work.

## Usage
- Follow the provided code structure and hints to build and integrate the various components of the blockchain system.
- Implement the work queue for parallel computation.
- Use the provided tests to validate the functionality of the work queue and the blockchain system.

## Conclusion
This project offers a hands-on experience in building a blockchain system with a focus on the proof-of-work concept. It combines data structures, cryptographic hashing, and parallel computing to create a functional blockchain model.
