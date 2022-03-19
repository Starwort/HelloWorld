use md5::Md5;
#[cfg(feature = "bad-idea")]
use rand::{rngs::ThreadRng, thread_rng, Rng};
use sha3::{Digest, Sha3_256};

#[cfg(feature = "reasonably-fast")]
const DECODE_SIZE: usize = 11;
#[cfg(not(feature = "reasonably-fast"))]
const DECODE_SIZE: usize = 36;

#[cfg(feature = "reasonably-fast")]
const TARGET_STRING: &[u8; DECODE_SIZE] = b"Hello world";
#[cfg(not(feature = "reasonably-fast"))]
const TARGET_STRING: &[u8; DECODE_SIZE] = b"Hello world proof of work edition :)";

struct AllCombinations {
    current_combination: [u8; DECODE_SIZE],
    #[cfg(not(feature = "bad-idea"))]
    initial: bool,
    #[cfg(feature = "bad-idea")]
    rng: ThreadRng,
}

impl AllCombinations {
    fn new() -> AllCombinations {
        AllCombinations {
            current_combination: [0; DECODE_SIZE],
            #[cfg(not(feature = "bad-idea"))]
            initial: true,
            #[cfg(feature = "bad-idea")]
            rng: thread_rng(),
        }
    }
}

impl Iterator for AllCombinations {
    type Item = [u8; DECODE_SIZE];

    #[cfg(not(feature = "bad-idea"))]
    fn next(&mut self) -> Option<Self::Item> {
        if self.initial {
            self.initial = false;
            return Some(self.current_combination);
        }
        for elem in self.current_combination.iter_mut() {
            if *elem == 0xff {
                *elem = 0;
                continue;
            }
            *elem += 1;
            return Some(self.current_combination);
        }
        None
    }
    #[cfg(feature = "bad-idea")]
    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(not(feature = "reasonably-fast"))]
        {
            self.rng.fill(&mut self.current_combination[..32]);
            self.rng.fill(&mut self.current_combination[32..]);
        }
        #[cfg(feature = "reasonably-fast")]
        self.rng.fill(&mut self.current_combination);
        Some(self.current_combination)
    }
}

fn main() {
    let hash_sha = Sha3_256::digest(TARGET_STRING);
    let hash_md5 = Md5::digest(TARGET_STRING);
    for comb in AllCombinations::new() {
        let computed_sha = Sha3_256::digest(comb);
        let computed_md5 = Md5::digest(comb);
        if computed_sha == hash_sha && computed_md5 == hash_md5 {
            let hello = &std::str::from_utf8(&comb).unwrap()[..11];
            println!("{}", hello);
            break;
        }
    }
}
