use std::collections::VecDeque;

pub enum CacheCode { HIT, MISS, EVICTION}

pub struct Set{
    lines: VecDeque<u64>,
    max_lines: usize,
    tag_offset: u32
}

impl Set{
    pub fn probe(&mut self, addr: u64) -> CacheCode{
        let tag = addr_to_tag(addr, self.tag_offset);
        for i in 0..self.lines.len(){
            if tag == self.lines[i] {
                self.lines.remove(i);
                self.lines.push_front(tag);
                return CacheCode::HIT;
            }
        }
        if self.lines.len() < self.max_lines{
            self.lines.push_front(tag);
            return CacheCode::MISS;
        }
        self.lines.pop_back();
        self.lines.push_front(tag);
        CacheCode::EVICTION
    }

    pub fn new(set_bits: u32, block_bits: u32, Assoc: u32) -> Set{
        Set{
            lines: VecDeque::new(),
            max_lines: Assoc as usize,
            tag_offset: block_bits + set_bits
        }
    }

}

pub fn addr_to_tag(addr: u64, tag_offset: u32) -> u64{
    let mask: u64 = !(!0b0 << (64 - tag_offset));
    mask & (addr >> tag_offset)
}

pub fn addr_to_setid(addr: u64, set_bits: u32, block_bits: u32) -> u64{
    let mask = !(!0b0 << set_bits);
    mask & (addr >> block_bits)
}
