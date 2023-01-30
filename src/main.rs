mod wordlist;

fn main() {
    let num_words: u8 = 6;

    let mut words: Vec<String> = Vec::new();

    for _ in 0..num_words {
        words.push(generate_word());
    }
    println!("{}", words.join(" "));
}

pub fn generate_word() -> String {
    let mut number: u32 = 0;

    for _ in 0..5 {
        let dice_roll = rand::random::<u32>() % 6 + 1;
        number *= 10;
        number += dice_roll;
    }

    wordlist::get_wordlist()
        .get_word(number)
        .unwrap()
        .to_string()
}
