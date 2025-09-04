mod components;

use crate::components::file_selector::FileSelector;
use iced::Length;
use iced::application::{Title, Update};
use iced::widget::{Row, button, center, checkbox, column, container, row, text, text_input};
use iced::{Application, Settings};
use iced::{Center, Element, Fill, Font, Task, Theme};
use rfd::{AsyncFileDialog, FileDialog};
use std::fmt::Debug;
use std::process::Command;
use std::sync::{Arc, Mutex};

pub fn main() -> iced::Result {
    iced::application("EasyASM", App::update, App::view)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .run()
}

#[derive(Default)]
struct App {
    source_file_location: String,
    output_directory: String,
}

#[derive(Clone)]
enum Message {
    Dynamic(Arc<Mutex<Option<Box<dyn FnOnce(&mut App) -> Task<Message> + Send>>>>),
    None,
}

impl Message {
    fn make_dynamic<'a>(f: impl FnOnce(&mut App) -> Task<Message> + Send + 'static) -> Self {
        Message::Dynamic(Arc::new(Mutex::new(Some(Box::new(f)))))
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Dynamic(_) => write!(f, "Message::Dynamic"),
            Message::None => write!(f, "Message::None"),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Dynamic(callback) => {
                let mut cb = callback.lock().unwrap();
                if let Some(f) = cb.take() {
                    f(self)
                } else {
                    Task::none()
                }
            }
            _ => Task::none(),
        }
    }

    async fn pick_psm_file() -> Option<String> {
        AsyncFileDialog::new()
            .add_filter("Assembly Files", &["psm"])
            .pick_file()
            .await
            .map(|file| file.path().to_string_lossy().to_string())
    }
    fn source_file_select(&self) -> Row<Message> {
        row![
            text_input("Source File", &self.source_file_location).on_input(|value| {
                Message::make_dynamic(move |app| {
                    app.source_file_location = value;
                    Task::none()
                })
            }),
            button("Browse").on_press(Message::make_dynamic(|app| {
                Task::perform(Self::pick_psm_file(), |path| {
                    path.map_or(Message::None, |p| {
                        Message::make_dynamic(move |app| {
                            app.source_file_location = p;
                            Task::none()
                        })
                    })
                })
            }))
        ]
    }

    async fn pick_output_directory() -> Option<String> {
        AsyncFileDialog::new()
            .pick_folder()
            .await
            .map(|folder| folder.path().to_string_lossy().to_string())
    }

    fn output_directory_select(&self) -> Row<Message> {
        row![
            text_input("Output Directory", &self.output_directory).on_input(|value| {
                Message::make_dynamic(move |app| {
                    app.output_directory = value;
                    Task::none()
                })
            }),
            button("Browse").on_press(Message::make_dynamic(|_app: &mut App| {
                Task::perform(Self::pick_output_directory(), |path| {

                    path.map_or(Message::None, |p| {
                        Message::make_dynamic(move |app| {
                            app.output_directory = p;
                            println!("Output Directory: {}", app.output_directory);
                            Task::none()
                        })
                    })
                })
            }))
        ]
    }

    fn view(&self) -> Element<Message> {
        container(column![self.source_file_select(), self.output_directory_select()].spacing(20))
            .padding(10)
            .into()
    }
}
