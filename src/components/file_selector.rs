use iced::Element;
use iced::widget::{button, column, container, row, text, text_input};

pub struct FileSelector {
    label: String,
    path: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    OnPathChanged(String),
    OnBrowse,
}

impl FileSelector {
    pub fn with_label(label: &str) -> Self {
        Self {
            label: label.to_string(),
            path: String::new(),
        }
    }

    pub fn with_label_and_default_path(label: &str, default_path: &str) -> Self {
        Self {
            label: label.to_string(),
            path: default_path.to_string(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let text_input: Element<Message> = text_input(
            &self.label,
            &self.path,
        ).on_input(Message::OnPathChanged).into();

        let browse_button: Element<Message> = button("Browse").on_press(Message::OnBrowse).into();

        row![text_input, browse_button].spacing(10).into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::OnPathChanged(new_path) => {
                self.path = new_path;
            }
            Message::OnBrowse => {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    if let Some(path_str) = path.to_str() {
                        self.path = path_str.to_string();
                    }
                }
            }
        }
    }
}