use std::{io::{Read, Result}, net::TcpStream, str::{from_utf8, Utf8Error}, vec};

use ratatui::{widgets::{Block, Borders, Paragraph}, DefaultTerminal, Frame};

#[derive(Clone, Debug)]
struct Data {
    buffer: Vec<u8>,
    msg_length: usize,
    input: String 
}

impl Data {
    fn msg_to_string(&self) -> &str {
        from_utf8(&self.buffer[0..self.msg_length]).expect("Could not convert buffer into string")
    }
}

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    
    let mut data = Data {
        buffer: vec![0; 64],
        msg_length: 0,
        input: String::new()
    };

    loop {
        data.msg_length = stream.read(&mut data.buffer).unwrap();

        terminal.draw(|f| {
            draw(f, data.clone());
        }).unwrap();
    }
}

fn draw(frame: &mut Frame, data: Data) {
    frame.render_widget(
        Paragraph::new(data.msg_to_string())
            .block(Block::new().borders(Borders::ALL)),
        frame.area());

    frame.render_widget(
        Paragraph::new(data.input)
            .block(Block::new().borders(Borders::ALL)),
        frame.area());
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    
    let result = run(&mut terminal);
    ratatui::restore();

    result
}
