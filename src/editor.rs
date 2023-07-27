use crate::Document;
use crate::Row;
use crate::Terminal;
use termion::event::Key;


const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    document: Document,
    cursor_position: Position,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            document: Document::open(),
            cursor_position: Position::default(),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }
        }
    }

    fn refresh_screen(&mut self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::cursor_position(&Position::default());

        if self.should_quit {
            Terminal::clear_screen();
            println!("Goodbye\r");
        } else {
            self.draw_rows();
            Terminal::cursor_position(&self.cursor_position);
        }
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::Up 
            | Key::Down 
            | Key::Left 
            | Key::Right 
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End
            => self.move_cursor(pressed_key),
            _ => println!("Key Pressed: {pressed_key:?}\r"),
        }
        Ok(())
    }

    fn move_cursor(&mut self, key: Key) {
        let Position{mut x, mut y} = self.cursor_position;
        let size = self.terminal.size();
        let height = size.height as usize;
        let width = size.width as usize;
        
        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1)
                }
            } 
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1)
                }
            } ,
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => ()
        }
        self.cursor_position =  Position{x, y}
    }

    fn draw_welcome_message(&self) {
        let mut welcome_msg = format!("MrCat Editor -- Version {}", VERSION);
        let width = self.terminal.size().width as usize; // Ex 30
        let len = welcome_msg.len();  // Ex: 10
        let padding = width.saturating_sub(len) / 2; // Ex: 10 (There will be 10 spaces padding per side)
        let spaces = " ".repeat(padding.saturating_sub(1)); // Ex: 9 spaces (See "~" char below)
        welcome_msg = format!("~{}{}", spaces, welcome_msg); // Ex "~"" + " " * 9
        welcome_msg.truncate(width);
        println!("{}\r", &welcome_msg)
    }

    pub fn draw_row(&self, row: &Row) {
        let start = 0;
        let end = self.terminal.size().width as usize;
        let row = row.render(start, end); // Need to implement
        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let height = self.terminal.size().height;

        for terminal_row in 0..height -1 {
            if let Some(row) = self.document.row(terminal_row as usize) {
                self.draw_row(row);
            } else if terminal_row == height / 3  {
                self.draw_welcome_message();
            } else {
                Terminal::clear_current_line();
                println!("~\r");
            }
        }
    }
}

fn die(e: std::io::Error) {
    Terminal::clear_screen();
    panic!("{}", e);
}
