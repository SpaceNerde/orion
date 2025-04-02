use anathema::{component::{Component, MouseState}, default_widgets::Overflow, prelude::{Document, TuiBackend}, runtime::{self, Runtime}, state::{List, State, Value} };

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

#[derive(Default, Debug)]
struct Messages {}

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
        todo!() // Work on rev of Messages from server and the sending of own messages
    }
}

fn main() {
    let doc = Document::new("@client");
    let backend = TuiBackend::builder()
        .enable_raw_mode()
        .enable_alt_screen()
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
        "messages",
        "templates/messages.aml",
        Messages::default(),
        MessagesState::default()
    ).unwrap();

    runtime.finish().unwrap().run();
}
