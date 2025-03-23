use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Layout, Line, Span, Terminal},
    widgets::{Block, List, ListItem, Paragraph},
    Frame,
};

use std::io::{stdout, BufRead, BufReader, ErrorKind, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

struct ClientApp {
    input: String,
    char_index: usize,
    messages: Vec<String>,
}

impl ClientApp {
    fn new() -> Self {
        Self {
            input: String::new(),
            char_index: 0,
            messages: vec![],
        }
    }

    fn move_cursor(&mut self, direction: i8) {
        match direction {
            -1 => {
                let cursor_move = self.char_index.saturating_sub(1);
                self.char_index = cursor_move.clamp(0, self.input.chars().count());
            }
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

    fn remove_char(&mut self) {
        self.input.pop();
        self.move_cursor(-1)
    }

    fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.char_index)
            .unwrap_or(self.input.len())
    }

    fn send_message(&mut self, stream: &mut TcpStream) {
        let message = format!("{}\n", self.input);
        stream
            .write_all(message.as_bytes())
            .expect("Could not write input to stream");

        self.input = String::new();
        self.char_index = 0;
    }
}

fn ui(f: &mut Frame, app: &ClientApp) {
    let layout = Layout::vertical([Constraint::Min(1), Constraint::Length(3)]);
    let [message_area, input_area] = layout.areas(f.size());

    let input = Paragraph::new(app.input.as_str()).block(Block::bordered().title("Input"));
    f.render_widget(input, input_area);

    let messages_items: Vec<ListItem> = app
        .messages
        .iter()
        .enumerate()
        .map(|(i, m)| {
            let content = Line::from(Span::raw(format!("{i}: {m}")));
            ListItem::new(content)
        })
        .collect();
    let messages_list = List::new(messages_items).block(Block::bordered().title("Messages"));
    f.render_widget(messages_list, message_area);
}

fn main() -> std::io::Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut client_app = ClientApp::new();

    match TcpStream::connect("127.0.0.1:8080") {
        Ok(mut stream) => {
            let (tx, rx) = mpsc::sync_channel(1);
            // message reciving thread
            let stream_clone = stream.try_clone().unwrap();

            thread::spawn(move || {
                let tx_clone = tx.clone();

                let mut reader = BufReader::new(&stream_clone);
                let mut buffer = String::new();

                loop {
                    match reader.read_line(&mut buffer) {
                        Ok(_) => {
                            let message = buffer.trim_end().to_string();
                            tx_clone.send(message).unwrap();
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
                if let Ok(message) = rx.try_recv() {
                    client_app.messages.push(message);
                }

                terminal.draw(|f| ui(f, &client_app))?;

                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Enter => client_app.send_message(&mut stream),
                            KeyCode::Char(to_insert) => {
                                client_app.enter_char(to_insert);
                            }
                            KeyCode::Backspace => {
                                client_app.remove_char();
                            }
                            KeyCode::Left => {
                                client_app.move_cursor(-1);
                            }
                            KeyCode::Right => {
                                client_app.move_cursor(1);
                            }
                            KeyCode::Esc => {
                                break;
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
