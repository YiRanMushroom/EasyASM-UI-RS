mod components;

use crate::Message::FileSelectorMessage;
use crate::components::file_selector::FileSelector;
use iced::widget::{button, column, container, row, text, text_input};
use iced::{Application, Settings};
use iced::{Center, Element, Fill, Font, Task, Theme};
use iced::application::{Title, Update};
use rfd::FileDialog;

struct State;

pub fn main() -> iced::Result {
    todo!()
}

struct EasyASMApp {
    file_selector: FileSelector,
}

#[derive(Debug, Clone)]
enum Message {
    FileSelectorMessage(components::file_selector::Message),
}

impl EasyASMApp {
    fn new() -> Self {
        Self {
            file_selector: FileSelector::with_label("Source File:"),
        }
    }

    fn view(&self) -> Element<Message> {
        // return view of file selector
        self.file_selector.view().map(FileSelectorMessage)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FileSelectorMessage(msg) => {
                self.file_selector.update(msg);
                Task::none()
            }
        }
    }
}

impl Title<State> for EasyASMApp {
    fn title(&self, state: &State) -> String {
        "EasyASM".to_string()
    }
}

impl Update<State, Message> for EasyASMApp {
    fn update(&self, state: &mut State, message: Message) -> impl Into<Task<Message>> {
        todo!()
    }
}
