use std::collections::HashMap;
use rand::prelude::*;

#[derive(Debug)]
struct Hand {
    card_one: u8,
    card_two: u8
}

impl Hand {
    fn new(used_cards: &mut Vec<u8>) -> Hand {
        let mut rng = thread_rng();
        let mut card: u8 = 0;

        let mut i: u8 = 0;
        while i < 2 {
            card = rng.gen_range(1..53);

            if used_cards.contains(&card) {
                println!("Card already used");
            } else {
                used_cards.push(card);
                i += 1;
            }
        }
        let len_used_cards = used_cards.len();
        return Hand {
            card_one: used_cards[len_used_cards - 1],
            card_two: used_cards[len_used_cards - 2]
        };
    }
}

fn main() {
    let mut used_cards: Vec<u8> = Vec::new();

    let player = Hand::new(&mut used_cards);
    let computer = Hand::new(&mut used_cards);

    println!("{:?}", player);
    println!("{:?}", computer);
    println!("{:?}", used_cards);
}
