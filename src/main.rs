mod wordlist;

fn main() {
    let num_words = get_num_words_via_dialog();
    let mut words: Vec<String> = Vec::new();

    for _ in 0..num_words {
        words.push(generate_word());
    }
    println!("{}", words.join(" "));
}

fn get_num_words_via_dialog() -> u8 {
    let choices = vec![
        "4 words (51 bits of entropy)",
        "5 words (64 bits of entropy)",
        "6 words (77 bits of entropy)",
    ];

    let selection = dialoguer::Select::new()
        .with_prompt("How strong do you want your passphrase to be?")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => 4,
        1 => 5,
        2 => 6,
        _ => 4,
    }
}

fn generate_word() -> String {
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
