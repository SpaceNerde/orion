use std::{io::{Read, Result}, net::TcpStream, str::from_utf8, vec};

use crossterm::event;
use ratatui::{widgets::{Block, Borders, Paragraph}, DefaultTerminal, Frame};

fn run(terminal: &mut DefaultTerminal) -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut buf = vec![0; 64];
    
    loop {
        let n = stream.read(&mut buf).unwrap();

        terminal.draw(|f| {
            draw(f, buf.clone(), n);
        }).unwrap();
        if matches!(event::read().unwrap(), event::Event::Key(_)) {
            break;
        }
    }

    Ok(())
}

fn draw(frame: &mut Frame, buf: Vec<u8>, n: usize) {
    let message = from_utf8(&buf[0..n]);

    frame.render_widget(
        Paragraph::new(message.unwrap())
            .block(Block::new().borders(Borders::ALL)),
        frame.area());
}

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    
    let result = run(&mut terminal);
    ratatui::restore();

    result
}
