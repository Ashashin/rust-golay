// (23,12,7) Golay code, syndrome decoding

fn get_syndrome(pattern: u32) -> u32 {
    // Returns the remainder of the polynomial division of pattern by the generating polynomial.

    let mut aux = 0x0040_0000; // X^22
    let mut aux2 = pattern;

    if pattern < 0x0000_0800 {
        // X^11
        return pattern;
    }

    while aux2 & 0xffff_f800 != 0 {
        while aux2 & aux == 0 {
            aux >>= 1;
        }
        aux2 ^= (aux / 0x0000_0800) * 0x0000_0c75;
    }
    aux2
}

fn main() {
    println!("Hello, world!");

    println!("Generating encoding table...");
    let mut encoding_table: [u32; 4096] = [0; 4096];
    for i in 0..4096 {
        let pattern = i;
        let tmp = (pattern << 11) as u32;
        encoding_table[i] = tmp + get_syndrome(tmp);
    }
    println!("Done.");

    println!("Testing.");
    // DATA contains 12 bits of information
    let data = 0x0000_0abc;

    println!("Data: {:032b}", data);

    // Codeword is 23 bits long
    let codeword = encoding_table[data as usize];
    println!("Codeword: {:032b}", codeword);
}
