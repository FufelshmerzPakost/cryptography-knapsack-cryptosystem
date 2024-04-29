use rand::Rng;
use bitvec::prelude::*;

fn gen_rand(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..=max)
}


fn private_key_gen(letter_count: usize) -> Vec<u32> {
    let mut private_key: Vec<u32> = Vec::new();
    private_key.push(gen_rand(1, 2));
    for _i in 0..letter_count-1 {
        let new_value: u32 = private_key[..].iter().sum::<u32>() + gen_rand(3, 4);
        private_key.push(new_value);
    }
    private_key
}

fn public_key_gen(n: u32, m: u32, private_key: Vec<u32>) -> Vec<u32> {
    private_key
    .iter()
    .map(|x| (x * n) % m)
    .collect::<Vec<u32>>()
}

fn encrypt_message(message: &str, public_key: Vec<u32>) -> Vec<u32> {
    let mut res: Vec<u32> = Vec::new();
    for i in message.chars() {
        let pos:BitVec<u8> = BitVec::from_element(i as u8);
        let mut encrypt_char: u32 = 0;
        for (idx, elem) in pos.iter().enumerate() {
            if elem == true {
                encrypt_char += public_key[idx];
            }
        }
        res.push(encrypt_char);
    }
    res
}

fn main() {
    let message: &str = "kriminalkishki";
    let private_key: Vec<u32> = private_key_gen(message.len());
    let n: u32 = 45;
    let m: u32 = gen_rand(*private_key.iter().max().unwrap(), 
    private_key.iter().max().unwrap() + 200);
    let public_key: Vec<u32> = public_key_gen(n, m, private_key.clone());
    let encrypt: Vec<u32> = encrypt_message(message, public_key.clone());
    println!("{:?}", message.len());
    println!("{:?}",private_key);
    println!("{:?}", public_key);
    println!("{:?}", encrypt);
} 
