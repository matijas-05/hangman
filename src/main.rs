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
    println!(
        r"
 _
| |
| |__   __ _ _ __   __ _ _ __ ___   __ _ _ __
| '_ \ / _` | '_ \ / _` | '_ ` _ \ / _` | '_ \
| | | | (_| | | | | (_| | | | | | | (_| | | | |
|_| |_|\__,_|_| |_|\__, |_| |_| |_|\__,_|_| |_|
                    __/ |
                   |___/"
    );

    let word = get_random_noun();
    const STAGES: [&str; 11] = [
        "",
        r"=========",
        r"
      |
      |
      |
      |
      |
=========",
        r"
  +---+
      |
      |
      |
      |
      |
=========",
        r"
  +---+
  |   |
      |
      |
      |
      |
=========",
        r"
  +---+
  |   |
  O   |
      |
      |
      |
=========",
        r"
  +---+
  |   |
  O   |
  |   |
      |
      |
=========",
        r"
  +---+
  |   |
  O   |
 /|   |
      |
      |
=========",
        r"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
=========",
        r"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
=========",
        r"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
=========",
    ];

    let mut current_stage = 0;
    let mut guessed_letters = Vec::<char>::new();

    loop {
        // Generate prompt
        let mut prompt = String::new();
        for c in word.chars() {
            if guessed_letters.contains(&c) {
                prompt.push(c);
            } else {
                prompt.push('_');
            }
        }

        // Win game
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
        // Lose game
        else if current_stage == STAGES.len() - 1 {
            execute!(
                stdout(),
                SetForegroundColor(Color::Red),
                Print(format!("You lost! The word was {}\n", word)),
                ResetColor
            )?;
            print!("{:?}", read()?);
            break;
        }

        // Print current stage and prompt
        execute!(stdout(), SavePosition, Print(STAGES[current_stage]))?;
        println!("\n{}\n\n", prompt);

        // Read input
        let mut input = String::new();
        print!("input: ");
        stdout().flush()?;
        stdin().read_line(&mut input)?;
        input = input.trim().to_owned();
        execute!(stdout(), Clear(ClearType::All), RestorePosition)?;
        if input.is_empty() {
            continue;
        };

        guessed_letters.push(input.chars().next().unwrap());

        // Next hangman graphic if guessed incorrectly
        if !word.contains(guessed_letters[guessed_letters.len() - 1]) {
            current_stage += 1;
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn get_random_noun() -> &'static str {
    let raw = include_str!("nouns.txt");
    let words = raw.split('\n').collect::<Vec<&str>>();
    let random_index = thread_rng().gen_range(0..words.len());

    words[random_index]
}
