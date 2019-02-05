extern crate rust_golay;

use rust_golay::syndrome::EncodingTable;

fn main() {
    println!("Hello, world!");
    println!("Generating encoding table...");
    
    let mut encoding_table = EncodingTable::new();
    encoding_table.gen_encoding_table();
    
    println!("Done.");
    println!("Testing.");
    // DATA contains 12 bits of information
    let data = 0x0000_0abc;

    println!("Data: {:032b}", data);

    // Codeword is 23 bits long
    let codeword = encoding_table.get_codeword(data);
    println!("Codeword: {:032b}", codeword);
}