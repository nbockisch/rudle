//! Functions to run the game logic

use std::{io::{BufReader, BufRead}, path::Path};
use rand::seq::SliceRandom;
use std::fs::File;

use crate::ui;

// Game parameters
const TITLE: &str = "RUDLE";
pub const NUM_CHARS: usize = 5;
const NUM_GUESSES: usize = 6;

// Stores if a char exists in the answer and if it is in the right place
#[derive(Debug, Clone)]
pub enum CharState {
    WRONG(char),
    EXISTS(char),
    CORRECT(char),
    EMPTY,
}

#[derive(Debug, Clone)]
pub struct GameComponent<T> {
    pub component: T,
    pub ypos: i32,
    pub xpos: i32,
}

///
/// Reset game elements so the game can be replayed
/// 
pub fn reset_game(prev_guesses: &mut Vec<GameComponent<Vec<CharState>>>, 
    guess_buf: &mut GameComponent<&mut String>, solution: &mut String) {

    for guess in prev_guesses {
        guess.component = vec![CharState::EMPTY; NUM_CHARS];
    }
    guess_buf.component.clear();

    let solutions: Vec<String> = get_words_from_file("src/solutions.txt");
    *solution = pick_solution(&solutions);
}

///
/// Read lines from a file and return them as a Vec of Strings
///
fn get_words_from_file(fname: impl AsRef<Path>) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();

    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    reader.lines().for_each(|line| {
        if let Ok(word) = line {
            words.push(word);
        }
    });

    words
}

///
/// Pick a random entry from a String Vec as the game solution
///
fn pick_solution(solutions: &Vec<String>) -> String {
    solutions.choose(&mut rand::thread_rng()).unwrap().to_string()
}

///
/// Initialize the game components and start the ui
///
pub fn run_game() {
    let mut title: GameComponent<&str> = GameComponent{
        component: TITLE,
        ypos: 0,
        xpos: 0,
    };

    let mut prev_guesses: Vec<GameComponent<Vec<CharState>>> = vec![
        GameComponent {
            component: vec![CharState::EMPTY; NUM_CHARS],
            ypos: 0,
            xpos: 0,
        } ; NUM_GUESSES
    ];

    let mut guess_buf: GameComponent<&mut String> = GameComponent {
        component: &mut String::new(),
        ypos: 0,
        xpos: 0,
    };

    let mut reset_btn: GameComponent<&String> = GameComponent {
        component: &String::from("[Reset]"),
        ypos: 0,
        xpos: 0,
    };

    let mut quit_btn: GameComponent<&String> = GameComponent {
        component: &String::from("[Quit]"),
        ypos: 0,
        xpos: 0,
    };

    let solutions: Vec<String> = get_words_from_file("src/solutions.txt");
    let all_words: Vec<String> = get_words_from_file("src/all-words.txt");
    let mut solution: String = pick_solution(&solutions);

    ui::run_ui(&mut title, &mut prev_guesses, &mut guess_buf, &mut reset_btn, 
        &mut quit_btn, &all_words, &mut solution);
}

///
/// Make sure the user's guess is a valid word
///
pub fn is_valid_guess(guess_buf: &String, all_words: &Vec<String>) -> bool {
    if (guess_buf.chars().count() != NUM_CHARS) || 
        (!all_words.contains(guess_buf)) {
        return false;
    }

    return true;
}

///
/// Check the correctness of a user's guess and build a CharState Vec with each
/// character's level of correctness
///
pub fn check_guess(guess_buf: &String,
    guess_field: &mut GameComponent<Vec<CharState>>, solution: &String) 
    -> bool {
    let mut is_won: bool = true;

    for (i, guess_char) in guess_buf.chars().enumerate() {
        let guess_field_char = guess_field.component.get_mut(i).unwrap();
        let answer_char: char = solution.chars().nth(i).unwrap_or('_');

        if guess_char == answer_char {
            *guess_field_char = CharState::CORRECT(guess_char);
        } else if solution.contains(guess_char) {
            *guess_field_char = CharState::EXISTS(guess_char);
            is_won = false;
        } else {
            *guess_field_char = CharState::WRONG(guess_char);
            is_won = false;
        }
    }

    is_won
}
