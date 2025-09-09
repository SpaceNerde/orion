use std::{io::{Read, Write}, net::TcpStream, result, str::from_utf8, sync::{mpsc::{channel, Receiver}, Arc}, thread::{self, sleep}, time::Duration};

use anathema::{component::{Component, KeyCode, MouseState}, default_widgets::Overflow, prelude::{Document, TuiBackend}, runtime::{self, Runtime}, state::{AnyState, List, State, Value}, widgets::components::events::KeyState};

type Result<T> = result::Result<T, ()>;

#[derive(Default, Debug)]
struct Client {}

impl Component for Client {
    type State = ();
    type Message = ();

    fn on_mouse(
            &mut self,
            mouse: anathema::component::MouseEvent,
            state: &mut Self::State,
            mut elements: anathema::widgets::Elements<'_, '_>,
            mut context: anathema::prelude::Context<'_, Self::State>,
        ) {
    
        elements.by_tag("overflow").first(|ov, _| {
            let overflow = ov.to::<Overflow>();
            match mouse.state {
                MouseState::ScrollUp => {
                    overflow.scroll_up();
                },
                MouseState::ScrollDown => {
                    overflow.scroll_down();
                },
                _ => ()
            }
        });
    }
}

#[derive(State, Default, Debug)]
struct MessagesState {
    messages: Value<List<String>>,
}

#[derive(Debug)]
struct Messages {
    ts: Duration,
    stream: Arc<TcpStream>,
}

impl Messages {
    fn new(stream: Arc<TcpStream>) -> Self {
        Self {
            ts: Duration::new(0, 0),
            stream
        }
    }
}

impl Component for Messages {
    type State = MessagesState;
    type Message = ();
    
    fn tick(
            &mut self,
            state: &mut Self::State,
            mut elements: anathema::widgets::Elements<'_, '_>,
            context: anathema::prelude::Context<'_, Self::State>,
            dt: std::time::Duration,
        ) {
        self.ts = self.ts.saturating_sub(dt);

        if self.ts != Duration::ZERO {
            return;
        }

        let mut buffer = vec![0; 64];
        let n = self.stream.as_ref().read(&mut buffer).expect("Something went bad when reading to buffer");

        state.messages.push(Value::new(from_utf8(&mut buffer[0..n]).expect("Could not turn into utf").to_string()));

        self.ts = Duration::from_secs(1);
    }
}

#[derive(Debug, Default, State)]
struct InputState {
    input_text: Value<List<char>>
}

#[derive(Debug)]
struct Input {
    stream: Arc<TcpStream>,
    input_buffer: Vec<u8>,
}

impl Input {
    fn new(stream: Arc<TcpStream>) -> Self {
        Self { 
            stream,
            input_buffer: vec![]
        }
    }
}

impl Component for Input {
    type State = InputState;
    type Message = ();

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        mut context: anathema::prelude::Context<'_, Self::State>,
    ) {
        match key.code {
            KeyCode::Char(c) => {
                self.input_buffer.push(c as u8);
                state.input_text.push_back(Value::new(c));
            },
            KeyCode::Enter => {
                drop(self.stream.as_ref().write(&self.input_buffer));
            },
            _ => (),
        }
    }
}

fn main() -> Result<()>{
    // TODO create a proper App flow and dont hardcode all that shit
    
    let stream = TcpStream::connect("127.0.0.1:8080").map_err(|e| {
        // dont panic but implement proper error handling!
        panic!("ERROR: Could not connect to server!")
    })?;
    let input_stream = stream.try_clone().unwrap();

    let doc = Document::new("@client");
    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_raw_mode()
        .enable_mouse()
        .finish()
        .unwrap();

    let mut runtime = Runtime::builder(doc, backend);
    
    runtime.register_component(
        "client",
        "templates/client.aml",
        Client::default(),
        ()
    ).unwrap();
    
    runtime.register_component(
        "input", 
        "templates/input.aml", 
        Input::new(Arc::new(input_stream)), 
        InputState::default()
    ).unwrap();

    runtime.register_component(
        "messages",
        "templates/messages.aml",
        Messages::new(Arc::new(stream)),
        MessagesState::default()
    ).unwrap();


    runtime.finish().unwrap().run();

    Ok(())
}
