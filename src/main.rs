use crossterm::{
    cursor::{RestorePosition, SavePosition},
    event::read,
    execute,
    style::*,
    terminal::*,
};
use rand::{prelude::thread_rng, Rng};
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    execute!(stdout(), EnterAlternateScreen)?;
    let word = get_random_noun();

    let mut guessed_letters = Vec::<char>::new();
    loop {
        // Display prompt
        let mut prompt = String::new();
        for c in word.chars() {
            if guessed_letters.contains(&c) {
                prompt.push(c)
            } else {
                prompt.push('_');
            }
        }

        // Win condition
        if !prompt.contains('_') {
            execute!(
                stdout(),
                SetForegroundColor(Color::Green),
                Print(format!("You won! The word was {}\n", word)),
                ResetColor
            )?;
            print!("{:?}", read()?);
            break;
        }

        execute!(stdout(), SavePosition, Print(format!("{}\n\n", prompt)))?;

        // Read line
        let mut input = String::new();
        print!("input: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        guessed_letters.push(input.chars().next().unwrap());

        execute!(stdout(), Clear(ClearType::All), RestorePosition)?
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn get_random_noun<'a>() -> &'a str {
    let raw = include_str!("nouns.txt");
    let words = raw.split('\n').collect::<Vec<&str>>();
    let random_index = thread_rng().gen_range(0..words.len());

    words[random_index]
}
