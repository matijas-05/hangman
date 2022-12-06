use rand::{prelude::thread_rng, Rng};

fn main() {
    println!("{}", get_random_noun());
}

fn get_random_noun() -> &'static str {
    let raw = include_str!("nouns.txt");
    let words = raw.split('\n').collect::<Vec<&str>>();
    let random_index = thread_rng().gen_range(0..words.len());

    words[random_index]
}
