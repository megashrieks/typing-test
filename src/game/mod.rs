use rand::prelude::*;
use rand;
use std::time;

use std::fs;

use std::io;
use std::io::Write;

use termion;
use termion::raw::IntoRawMode;
use termion::input::{ TermRead, Keys };
use termion::AsyncReader;

fn getchar(stdin: &mut Keys<AsyncReader>) -> Option<termion::event::Key> {
    let mut what_count = 0;
    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Backspace => {
                    return Some(termion::event::Key::Backspace);
                }
                termion::event::Key::Char(_) => {
                    return Some(key);
                }
                _ => {
                    what_count += 1;
                    if what_count == 3 {
                        return None;
                    }
                }
            }
        }
    }
}

fn print_word_list(word_list: &String, flag: &Vec<bool>) {
    for (index, character) in word_list.char_indices() {
        if index < flag.len() { 
            if flag[index] {
                print!("{}{}", termion::color::Fg(termion::color::Green), character);
            } else {
                print!("{}{}", termion::color::Fg(termion::color::Red), character);
            };
        } else if index == flag.len(){
            print!(
                "{}{}{}{}", 
                termion::style::Underline,
                termion::color::Fg(termion::color::Reset),
                character,
                termion::style::NoUnderline
            );
        } else {
            print!("{}{}", termion::color::Fg(termion::color::Reset), character);
        }
    }
}

fn get_words() -> Vec<String> {
    const TOP1KPATH: &str = "./src/assets/top1kwords.txt";
    let content: String = fs::read_to_string(TOP1KPATH).expect("should have returned the file content");
    let split_content: Vec<String> = content.split('\n').map(|s: &str| String::from(s)).collect();
    return split_content;
}

fn generate_test_words(all_words: &Vec<String>) -> String {
    const TOTAL_TEST_WORDS:i32 = 40;
    let mut rng = rand::thread_rng();
    let mut result: String = String::new();
    for _i in 0..TOTAL_TEST_WORDS {
        let r = rng.gen_range(0..(all_words.len()));
        result += &format!("{} ", all_words[r]);
    }
    result.pop().unwrap();
    result
}

pub fn gameloop() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let word_list: Vec<String> = get_words();
    let mut test_words: String = generate_test_words(&word_list);
    let mut flag: Vec<bool> = Vec::new();
    let mut current_flag = true;
    let mut words_completed = 0;
    let mut start_time = time::Instant::now();
    loop {
        let (x, y) = termion::terminal_size().unwrap();

        write!(stdout, "{}", termion::clear::All).unwrap();
        writeln!(stdout, "{}", termion::cursor::Goto(1,1)).unwrap();
        print_word_list(&test_words, &flag);
        let time_elapsed = start_time.elapsed().as_secs();
        let wpm = words_completed * 60 / (time_elapsed + 1) as usize;

        writeln!(stdout, "{}", termion::cursor::Goto(1,y-3)).unwrap();
        writeln!(stdout, "Time Elapsed: {} secs", time_elapsed).unwrap();

        writeln!(stdout, "\rWords Per Minute: {}", wpm).unwrap();


        let cursor_line: u16 = flag.len() as u16 / x;
        let current_column: u16 = flag.len() as u16 % x;
        writeln!(stdout, "\r{}", termion::cursor::Goto(current_column+1,cursor_line+1)).unwrap();

        let char = getchar(&mut stdin);
        if let Some(termion::event::Key::Char(character)) = char {
            if let Some(nthchar) = test_words.chars().nth(flag.len()) {
                if character.eq(&nthchar) {
                    flag.push(current_flag);
                    current_flag = true;
                } else {
                    current_flag = false;
                }
            }
            let split = test_words[0..(flag.len())].split(' ');
            words_completed = split.collect::<Vec<&str>>().len()
        } else if let Some(termion::event::Key::Backspace) = char {
        } else {
            write!(stdout, "{}", termion::cursor::Goto(1,1)).unwrap();
            writeln!(stdout, "{}", termion::clear::All).unwrap();
            println!("Invalid key; exiting");
            break;
        }
        if flag.len() == test_words.len() {
            test_words.clear();
            flag.clear();
            current_flag = true;
        }
        if test_words.is_empty() {
            start_time = time::Instant::now();
            test_words = generate_test_words(&word_list);
        }

        write!(stdout, "{}", termion::clear::All).unwrap();
        stdout.lock().flush().unwrap();
    }
}
