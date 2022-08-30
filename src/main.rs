use std::collections::HashMap;
use rand::prelude::*;

#[derive(Debug)]
struct Hand {
    card_one: u8,
    card_two: u8
}

impl Hand {
    fn new() -> Hand {
        let mut rng = thread_rng();
        let mut first_card: u8 = 0;
        let mut second_card: u8 = 0;

        let mut i: u8 = 0;
        while i < 1 {
            first_card = rng.gen_range(1..53);
            second_card = rng.gen_range(1..53);

            if first_card != second_card {
                i += 1;
            }
        }
        return Hand { card_one: first_card, card_two: second_card };
    }
}

fn main() {
    let used_cards: Vec<u8> = Vec::new();

    // Compare structs with PartialEq: https://turreta.com/2019/09/15/rust-how-to-compare-struct-instances/
    let player = Hand::new();
    let computer = Hand::new();


    println!("{:?}", player);
    println!("{:?}", computer);
}
