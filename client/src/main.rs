use std::{io::{self, BufRead, BufReader, Read, Write}, net::TcpStream, result, str::from_utf8, sync::{Arc, mpsc::{Receiver, channel}}, thread::{self, sleep}, time::Duration};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, prelude::*, symbols::border, widgets::{Block, Paragraph}};

#[derive(Debug)]
pub struct Client {
    exit: bool,
    stream: TcpStream,
    buffer: Vec<u8>,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self { 
            exit: false, 
            stream, 
            buffer: vec![], 
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut reader = io::BufReader::new(self.stream.try_clone().unwrap());

        while !self.exit {
            self.buffer = reader.fill_buf()?.to_vec();

            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit(),
            _ => ()
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &Client {
    fn render(self, area: Rect, buf: &mut Buffer) where Self: Sized {
        let title = Line::from(" Orion Client ").bold();

        let block = Block::bordered()
            .title(title.centered())
            .border_set(border::THICK);

        Paragraph::new(from_utf8(&self.buffer).unwrap())
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080").map_err(|e| {
        // dont panic but implement proper error handling!
        panic!("ERROR: Could not connect to server!")
    }).unwrap();

    let mut input_stream = stream.try_clone().unwrap();


    let mut terminal = ratatui::init();
    let app_result = Client::new(input_stream).run(&mut terminal);
    ratatui::restore();

    app_result
}
