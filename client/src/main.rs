use std::io::{stdin, BufRead, BufReader, ErrorKind, Write, stdout};
use std::net::TcpStream;
use std::thread;
use crossterm::{
    event::{self, KeyCode, KeyEventKind, Event},
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::{Paragraph, Borders, Block},
    Frame
};

struct ClientApp {
    input: String,
    char_index: usize,
}

impl ClientApp {
    fn new() -> Self {
        Self {
            input: String::new(),
            char_index: 0,
        }
    }

    fn move_cursor(&mut self, direction: i8) {
        match direction {
            -1 => {
                let cursor_move = self.char_index.saturating_sub(1);
                self.char_index = cursor_move.clamp(0, self.input.chars().count());
            },
            1 => {
                let cursor_move = self.char_index.saturating_add(1);
                self.char_index = cursor_move.clamp(0, self.input.chars().count());
            }
            _ => {
                panic!("Direction can only range from -1 to 1!");
            }
        }
    }

    fn enter_char(&mut self, io_char: char) {
        let index = self.byte_index();
        self.input.insert(index, io_char);
        self.move_cursor(1);
    }

    fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.char_index)
            .unwrap_or(self.input.len())
    }

    fn send_message(&mut self, stream: &mut TcpStream) {
        stream.write(self.input.as_bytes());
        self.input = String::new();
        self.char_index = 0;
    }
}

fn main() -> std::io::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut client_app = ClientApp::new();

    match TcpStream::connect("127.0.0.1:80") {
        Ok(mut stream) => {
            // message reciving thread
            let stream_clone = stream.try_clone().unwrap();

            thread::spawn(move || {
                let mut reader = BufReader::new(&stream_clone);
                let mut buffer = String::new();

                loop {
                    match reader.read_line(&mut buffer) {
                        Ok(_) => {
                            let message = buffer.trim_end().to_string();
                            println!("{:?}", message);
                            buffer.clear();
                        }
                        Err(e) if e.kind() == ErrorKind::ConnectionAborted => {
                            println!("INFO: Connection to server lost.");
                            break;
                        }
                        Err(e) if e.kind() == ErrorKind::ConnectionReset => {
                            println!("INFO: Connection to server lost.");
                            break;
                        }
                        Err(e) => {
                            panic!("ERROR: {:?}", e);
                        }
                    }
                }
            });

            // Main Loop Cause
            loop {
                terminal.draw(|f| ui(f, &client_app))?;

                if let Event::Key(key) = event::read()? { 
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Enter => client_app.send_message(&mut stream),
                            KeyCode::Char(to_insert) => {
                                client_app.enter_char(to_insert);
                            }
                            KeyCode::Backspace => {
                                // TODO
                            }
                            KeyCode::Left => {
                                client_app.move_cursor(-1);
                            }
                            KeyCode::Right => {
                                client_app.move_cursor(1);
                            }
                            KeyCode::Esc => {
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        Err(_) => {
            // You dont always have to panic if something goes wrong ;)
            println!("ERROR: Could not connect to server!");
        }
    }   

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &ClientApp) {
    let input = Paragraph::new(app.input.as_str()).block(Block::bordered().title("Input"));
    f.render_widget(input, f.size());
}

