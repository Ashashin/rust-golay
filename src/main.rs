extern crate rust_golay;

use rust_golay::syndrome::gen_encoding_table;

fn main() {
    println!("Hello, world!");
    println!("Generating encoding table...");
    
    let encoding_table: [u32; 4096] = gen_encoding_table();
    
    println!("Done.");
    println!("Testing.");
    // DATA contains 12 bits of information
    let data = 0x0000_0abc;

    println!("Data: {:032b}", data);

    // Codeword is 23 bits long
    let codeword = encoding_table[data as usize];
    println!("Codeword: {:032b}", codeword);
}