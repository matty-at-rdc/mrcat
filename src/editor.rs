use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;



pub struct Editor {}

impl Editor {

    pub fn default() -> Self {
        Self{}
    }

    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn process_keypress(&self) -> Result<(), std::io::Error> {
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Program End"),
            _ => println!("Key Pressed: {pressed_key:?}\r"),
        }
        Ok(())
    }

}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}

fn die(e: std::io::Error) {
    panic!("{}", e);
}


// OLD
// for key in io::stdin().keys() {
//     match key {
//         Ok(key) => match key {
//             Key::Char(c) => {
//                 if c.is_control() {
//                     println!("{:?}\r", c as u8);
//                 } else {
//                     println!("{:?} ({})\r", c as u8, c);
//                 }
//             }

//             Key::Ctrl('q') => break,

//             _ => println!("{key:?}\r"),
//         },
//         Err(err) => die(err),
//     }
// }