use crate::{println, print, clearrow, shell};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::StreamExt;
use crate::task::{Task, executor::Executor};
use alloc::{
    string::String,
    format
};

pub fn init(executor: &mut Executor) {
    executor.spawn(Task::new(shell::main()));
    print_ps(1);
}

pub fn print_ps(ps: u8) {
    print!("{}", match ps {
        1 => "$ ",
        2 => "> ",
        _ => "?"
    })
}

pub async fn main() {
    let mut scancodes = crate::task::keyboard::ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
        HandleControl::Ignore);
    let mut input = String::new();

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => {
                        match character {
                            '\n' => {
                                println!();
                                match input.as_str() {
                                    "help" => {
                                        println!("There is no help.");
                                    },
                                    "hello" => {
                                        println!("Hello there!");
                                    },
                                    "about" => {
                                        println!("K I N D A _ O S\nno version number :)\ncreated by TaiAurori#6781\nlargely programmed from os.phil-opp.com");
                                    },
                                    _ => {
                                        println!("Unrecognized command.");
                                    }
                                }
                                input = String::new();
                            }, 
                            '\u{0008}' => {
                                if input.len() > 0 {
                                    input.truncate(input.len() - 1);
                                }
                            },
                            _ => {
                                input = format!("{}{}", input, character);
                            }
                        }
                        clearrow!();
                        print_ps(1);
                        print!("{}", input);
                    },
                    DecodedKey::RawKey(key) => {
                        
                    },
                }
            }
        }
    }
}