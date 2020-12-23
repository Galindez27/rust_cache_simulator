use std::collections::VecDeque;
use std::u64;

pub enum CacheCode { HIT, MISS, EVICTION}

pub struct CacheOutput{
    hits: u32,
    misses: u32,
    evictions: u32,
    total_probes: u32
}

impl CacheOutput{
    pub fn new() -> CacheOutput{
        CacheOutput {
            hits: 0,
            misses: 0,
            evictions: 0,
            total_probes: 0
        }
    }
}

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

    pub fn new(set_bits: u32, assoc: u32, block_bits: u32, ) -> Set{
        Set{
            lines: VecDeque::new(),
            max_lines: assoc as usize,
            tag_offset: block_bits + set_bits
        }
    }

}

pub struct CacheSim{
    sets: Vec<Set>,
    total_sets: usize,
    outputs: CacheOutput,
    set_bits: u32,
    block_bits: u32
}

impl CacheSim{
    pub fn new(s: u32, E: u32, b: u32) -> CacheSim {
        let tsets: usize = 1 << s;
        let mut sets = Vec::with_capacity(tsets);
        for _i in 0..tsets{
            sets.push(Set::new(s, E, b));
        }
        return CacheSim{
            sets,
            total_sets: tsets,
            outputs: CacheOutput::new(),
            set_bits: s,
            block_bits: b
        }
    }

    pub fn probe_cache(&mut self, addr: u64) -> CacheCode{
        let set_id: usize = addr_to_setid(addr, self.set_bits, self.block_bits);
        let set = self.sets.get_mut(set_id).expect("Failed to get set!");
        // self.outputs.total_probes = self.outputs.total_probes + 1;
        set.probe(addr)
    }

    pub fn check_trace_file(&mut self, filepath: &str) {
        use std::fs;
        let file_str = fs::read_to_string(filepath).expect("Failed to open the file!");
        let lines: Vec<&str> = file_str.split('\n').collect();

        for line in lines{
            let trimmed = line.trim();
            if trimmed.starts_with('I') { continue }
            
            let modify = trimmed.starts_with('M') as u32;
            let spl = trimmed.split(',').next().unwrap();
            let spl1: Vec<&str> = spl.split(' ').collect();
            
            let addr: u64 = u64::from_str_radix(spl1[1], 16).expect("Failed to parse address");
            for _ in 0..modify{
                match self.probe_cache(addr){
                    CacheCode::HIT => self.outputs.hits = self.outputs.hits + 1,
                    CacheCode::MISS => self.outputs.misses = self.outputs.misses + 1,
                    CacheCode::EVICTION => {
                        self.outputs.misses = self.outputs.misses + 1;
                        self.outputs.evictions = self.outputs.evictions + 1;
                    }
                }
            }
        }
    }
}

pub fn addr_to_tag(addr: u64, tag_offset: u32) -> u64{
    let mask: u64 = !(!0b0 << (64 - tag_offset));
    mask & (addr >> tag_offset)
}

pub fn addr_to_setid(addr: u64, set_bits: u32, block_bits: u32) -> usize{
    let mask = !(!0b0 << set_bits);
    (mask & (addr >> block_bits)) as usize
}





#[test]
fn five_one_five_long() {
    let mut cache_sim = CacheSim::new(5, 1, 5);
    cache_sim.check_trace_file("traces/long.trace");
    let answers = CacheOutput{
        hits: 265189,
        misses: 21775,
        evictions: 21743,
        total_probes: 0
    };

    assert_eq!(cache_sim.outputs.hits, answers.hits);
    assert_eq!(cache_sim.outputs.misses, answers.misses);
    assert_eq!(cache_sim.outputs.evictions, answers.evictions);
}