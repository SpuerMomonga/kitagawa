use iced::executor;
use iced::{Application, Command, Element, Theme};

struct Search {}

impl Application for Search {
    type Executor = executor::Default;
    type Flags = ();
    type Message = ();
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> Element<Self::Message> {
        todo!()
    }
}
