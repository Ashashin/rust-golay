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

pub fn gen_encoding_table() -> [u32; 4096] {
  let mut encoding_table: [u32; 4096] = [0; 4096];

  for i in 0..4096 {
    let pattern = i;
    let tmp = (pattern << 11) as u32;
    encoding_table[i] = tmp + get_syndrome(tmp);
  }

  encoding_table
}