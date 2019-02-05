
pub struct EncodingTable {
    table: [u32; 4096],
}


impl EncodingTable {

    pub fn new () -> Self {
        Self { table: [0; 4096] }
    }

    pub fn gen_encoding_table(&mut self) {
        for i in 0..4096 {
            let pattern = i;
            let tmp = (pattern << 11) as u32;
            self.table[i] = tmp + self.get_syndrome(tmp);
        }
    }

    pub fn get_codeword(&self, codeword: u32) -> u32 {
        self.table[codeword as usize]
    }

    fn get_syndrome(&self, pattern: u32) -> u32 {
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

}
// (23,12,7) Golay code, syndrome decoding
