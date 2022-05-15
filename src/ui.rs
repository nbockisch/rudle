//! Functions to run UI components

use std::process;
use ncurses;
use crate::game;

///
/// Display previous guess to the screen, showing the correctness of each letter
/// with different colors
///
fn display_guess(guess: &game::GameComponent<Vec<game::CharState>>) {
    ncurses::init_pair(1, ncurses::COLOR_WHITE, ncurses::COLOR_BLACK);
    ncurses::init_pair(2, ncurses::COLOR_BLACK, ncurses::COLOR_YELLOW);
    ncurses::init_pair(3, ncurses::COLOR_BLACK, ncurses::COLOR_GREEN);

    ncurses::mv(guess.ypos, guess.xpos);
    ncurses::clrtoeol();

    for letter in &guess.component {
        let mut disp_char: char = '_';
        let mut color_code: i16 = 1;

        match letter {
            game::CharState::EMPTY => {}
            game::CharState::WRONG(c) => disp_char = *c,
            game::CharState::EXISTS(c) => {
                disp_char = *c;
                color_code = 2;
            },
            game::CharState::CORRECT(c) => {
                disp_char = *c;
                color_code = 3;
            }
        }

        ncurses::attron(ncurses::COLOR_PAIR(color_code));
        ncurses::addch(disp_char as u32);
        ncurses::attroff(ncurses::COLOR_PAIR(color_code));
    }

}

///
/// Update the user guess text on the screen
///
fn update_guess_buf(guess_buf: &mut game::GameComponent<&mut String>,
    input_char: i32) {
    match input_char {
        ncurses::KEY_BACKSPACE | 127  => {
            guess_buf.component.pop();
        },
        // Make sure input is a letter
        65..=122 => {
            // Convert to lower case
            let mut lower_char: u32 = input_char as u32;
            if lower_char <= 90 {lower_char += 32}

            if guess_buf.component.chars().count() < game::NUM_CHARS {
                guess_buf.component.push(
                    std::char::from_u32(lower_char).unwrap());
            }
        },
        _ => {},
    }
    ncurses::mv(guess_buf.ypos, guess_buf.xpos);
    ncurses::clrtoeol();
    ncurses::addstr(guess_buf.component);
}

///
/// Draw all the menu items to the screen
///
fn render_menu(menu_items: &Vec<&game::GameComponent<&String>>, 
    sel_xpos: i32) {

    for item in menu_items {
        if item.xpos == sel_xpos {ncurses::attron(ncurses::A_BOLD());}

        ncurses::mvaddstr(item.ypos, item.xpos, &item.component.to_string());
        ncurses::attroff(ncurses::A_BOLD());
    }
}

///
/// Run the menu ui
///
fn run_menu(prev_guesses: &mut Vec<game::GameComponent<Vec<game::CharState>>>, 
    guess_buf: &mut game::GameComponent<&mut String>, 
    reset_btn: &game::GameComponent<&String>, 
    quit_btn: &game::GameComponent<&String>, all_words: &Vec<String>,
    solution: &mut String) -> bool {

    ncurses::mvaddstr(reset_btn.ypos, reset_btn.xpos, reset_btn.component);
    ncurses::mvaddstr(quit_btn.ypos, quit_btn.xpos, quit_btn.component);

    let menu_items: Vec<&game::GameComponent<&String>> = 
        vec![reset_btn, quit_btn];
    // vars for determining menu state
    let mut sel_ypos: i32 = guess_buf.ypos;
    let mut sel_xpos: i32 = guess_buf.xpos;
    let mut input_ch: i32;
    let mut cur_guess: usize = 0;
    let mut is_won: bool = false;

    // Navigate the menu with the arrow keys and use RETURN to select items
    ncurses::keypad(ncurses::stdscr(), true);
    loop {
        render_menu(&menu_items, sel_xpos);

        if (is_won) || (cur_guess >= prev_guesses.len()) {
            display_text(solution, guess_buf.ypos + 1, guess_buf.xpos);
        }

        input_ch = ncurses::getch();
        match input_ch {
            // Switch between the button row and text input area
            ncurses::KEY_UP | ncurses::KEY_DOWN => {
                if sel_ypos == guess_buf.ypos {
                    sel_ypos = reset_btn.ypos;
                    sel_xpos = reset_btn.xpos;
                } else {
                    sel_ypos = guess_buf.ypos;
                    sel_xpos = guess_buf.xpos;
                }
            },
            // Switch between reset and quit buttons
            ncurses::KEY_LEFT | ncurses::KEY_RIGHT => {
                if sel_xpos == reset_btn.xpos {
                    sel_xpos = quit_btn.xpos;
                } else if sel_xpos == quit_btn.xpos {
                    sel_xpos = reset_btn.xpos;
                }
            },
            // Execute menu item or check user guess
            ncurses::KEY_ENTER | 10 => {
                match sel_xpos {
                    x if x == quit_btn.xpos => break,
                    x if x == reset_btn.xpos => {
                        game::reset_game(prev_guesses, guess_buf, solution);
                        return true;
                    },
                    x if x == guess_buf.xpos => {
                        if (!game::is_valid_guess(guess_buf.component, 
                            all_words)) || (cur_guess >= prev_guesses.len())
                            || (is_won) {

                            continue;
                        }

                        is_won = game::check_guess(guess_buf.component, 
                            prev_guesses.get_mut(cur_guess).unwrap(), solution);

                        display_guess(prev_guesses.get(cur_guess).unwrap());
                        cur_guess += 1;

                        guess_buf.component.clear();
                        update_guess_buf(guess_buf, 0);

                        if is_won || cur_guess >= prev_guesses.len() {
                            sel_ypos = reset_btn.ypos;
                            sel_xpos = reset_btn.xpos;
                        }
                    },
                    _ => {},
                }
            },
            // Input guess text
            _ => {
                sel_ypos = guess_buf.ypos;
                sel_xpos = guess_buf.xpos;
                if (!is_won) && (cur_guess < prev_guesses.len()) {
                    update_guess_buf(guess_buf, input_ch); 
                }
            },
        }
    }

    return false;
}

///
/// Helper function to display a string at a given position
///
fn display_text(text: &String, ypos: i32, xpos: i32) {
    ncurses::mv(ypos, xpos);
    ncurses::clrtoeol();
    ncurses::addstr(text);
}

///
/// Display the ui components and reset the ui on game reset
///
pub fn run_ui(title: &mut game::GameComponent<&str>, 
    prev_guesses: &mut Vec<game::GameComponent<Vec<game::CharState>>>, 
    guess_buf: &mut game::GameComponent<&mut String>, 
    reset_btn: &mut game::GameComponent<&String>, 
    quit_btn: &mut game::GameComponent<&String>, all_words: &Vec<String>,
    solution: &mut String) {

    let mut should_reset = true;

    while should_reset {
        // UI parameters and components
        let (mut row, mut col): (i32, i32) = (0, 0);
        let num_components: i32 = 2 + prev_guesses.len() as i32;

        ncurses::initscr();
        ncurses::clear();
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        // Make sure terminal will support this
        if !ncurses::has_colors() {
            eprintln!("FATAL: Terminal doesn't support colors!");
            process::exit(1);
        }
        ncurses::start_color();

        // Get terminal dimensions
        ncurses::getmaxyx(ncurses::stdscr(), &mut row, &mut col);

        // Display UI components
        title.ypos = (row - num_components) / 2;
        title.xpos = (col - title.component.chars().count() as i32) / 2;
        ncurses::mvaddstr(title.ypos, title.xpos, title.component);
        ncurses::mvaddstr(title.ypos + 1, title.xpos, 
            &"=".repeat(title.component.chars().count()));

        reset_btn.ypos = title.ypos + 2;
        reset_btn.xpos = (col / 2) - reset_btn.component.chars().count() as i32 
            - 1;
        ncurses::mvaddstr(reset_btn.ypos, reset_btn.xpos, reset_btn.component);

        quit_btn.ypos = title.ypos + 2;
        quit_btn.xpos = (col / 2) + 1;
        ncurses::mvaddstr(quit_btn.ypos, quit_btn.xpos, quit_btn.component);

        for (index, guess_field) in prev_guesses.iter_mut().enumerate() {
            guess_field.ypos = quit_btn.ypos + 1 + index as i32;
            guess_field.xpos = (col - guess_field.component.len() as i32) / 2;
            display_guess(guess_field);
        }

        guess_buf.ypos = quit_btn.ypos + prev_guesses.len() as i32 + 2;
        guess_buf.xpos = (col - game::NUM_CHARS as i32) / 2;

        should_reset = run_menu(prev_guesses, guess_buf, reset_btn,
            quit_btn, all_words, solution);

        ncurses::refresh();
        ncurses::endwin();
    }
}
