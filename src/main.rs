#![windows_subsystem = "windows"]

use iced::application::{Title, Update};
use iced::widget::{button, column, container, row, text_input};
use iced::{Element, Task};
use rfd::AsyncFileDialog;
use std::fmt::Debug;
use std::process::Command;
use std::sync::{Arc, Mutex};

pub fn main() -> iced::Result {
    iced::application("EasyASM", App::update, App::view).run()
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
    fn source_file_select(&self) -> Element<Message> {
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
        .into()
    }

    async fn pick_output_directory() -> Option<String> {
        AsyncFileDialog::new()
            .pick_folder()
            .await
            .map(|folder| folder.path().to_string_lossy().to_string())
    }

    fn output_directory_select(&self) -> Element<Message> {
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
                            Task::none()
                        })
                    })
                })
            }))
        ]
        .into()
    }

    async fn pop_message(title: &str, message: &str) {
        rfd::AsyncMessageDialog::new()
            .set_title(title)
            .set_description(message)
            .show()
            .await;
    }

    fn compile_button(&self) -> Element<Message> {
        button("Compile")
            .on_press(Message::make_dynamic(|app| {
                let source_file_location = app.source_file_location.clone();
                let output_directory = app.output_directory.clone();
                Task::perform(
                    async move {
                        App::do_compile(source_file_location, output_directory).await
                    },
                    |result| {
                        match result {
                            ExecutionResult::WithErrorCode(code, message) => {
                                let error_message = format!("Compiler crashed with error code: {}\n\nMessage: {:?}", code, message);
                                Message::make_dynamic(move |_app| {
                                    Task::perform(
                                        async move {
                                            App::pop_message("Error", &error_message).await;
                                        },
                                        |_| Message::None,
                                    )
                                })
                            }
                            ExecutionResult::WithExecutionError(err) => {
                                let error_message = format!("Failed to execute compiler: {}", err);
                                Message::make_dynamic(move |_app| {
                                    Task::perform(
                                        async move {
                                            App::pop_message("Execution Error", &error_message).await;
                                        },
                                        |_| Message::None,
                                    )
                                })
                            }
                            ExecutionResult::WithOutput(output) => {
                                match (output.stdout, output.stderr) {
                                    (Some(stdout), Some(stderr)) => {
                                        let message = format!("Compilation succeeded.\n\nOutput:\n{}\n\nPossible Errors:\n{}", stdout, stderr);
                                        Message::make_dynamic(move |_app| {
                                            Task::perform(
                                                async move {
                                                    App::pop_message("Compilation Result", &message).await;
                                                },
                                                |_| Message::None,
                                            )
                                        })
                                    }
                                    (Some(stdout), None) => {
                                        let message = format!("{}", stdout);
                                        Message::make_dynamic(move |_app| {
                                            Task::perform(
                                                async move {
                                                    App::pop_message("Compilation Succeed", &message).await;
                                                },
                                                |_| Message::None,
                                            )
                                        })
                                    }
                                    (None, Some(stderr)) => {
                                        let message = format!("{}", stderr);
                                        Message::make_dynamic(move |_app| {
                                            Task::perform(
                                                async move {
                                                    App::pop_message("Compilation Failed", &message).await;
                                                },
                                                |_| Message::None,
                                            )
                                        })
                                    }
                                    (None, None) => {
                                        Message::make_dynamic(move |_app| {
                                            Task::perform(
                                                App::pop_message("Compilation Result", "Compilation completed with no output."),
                                                |_| Message::None,
                                            )
                                        })
                                    }
                                }
                            },
                            _ => Message::None
                        }
                    }
                )
            }))
            .into()
    }

    fn view(&self) -> Element<Message> {
        container(
            column![
                self.source_file_select(),
                self.output_directory_select(),
                self.compile_button()
            ]
            .spacing(20),
        )
        .padding(10)
        .into()
    }

    async fn do_compile(
        self_source_file_location: String,
        self_output_directory: String,
    ) -> ExecutionResult {
        let source_file_location: String;
        let output_directory: Option<String>;

        if self_source_file_location.is_empty() {
            return ExecutionResult::WithExecutionError(
                "Source file location is empty".to_string(),
            );
        } else {
            source_file_location = self_source_file_location.clone();
        }

        if self_output_directory.is_empty() {
            output_directory = None;
        } else {
            output_directory = Some(self_output_directory.clone());
        }

        let mut args: Vec<String> = Vec::new();
        args.push("-l".to_string());

        let PicoBlaze_path = std::env::current_dir().unwrap().join("PicoBlaze");

        args.push(PicoBlaze_path.to_string_lossy().to_string());
        args.push("-i".to_string());
        args.push(source_file_location);

        if let Some(output_dir) = output_directory {
            args.push("-o".to_string());
            args.push(output_dir);
        }

        execute_program(
            "EasyASM.exe",
            &args.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
        )
        .await
    }
}

struct StdoutOutput {
    stdout: Option<String>,
    stderr: Option<String>,
}

enum ExecutionResult {
    WithErrorCode(i32, Option<String>),
    WithExecutionError(String),
    WithOutput(StdoutOutput),
    None,
}

async fn execute_program(command: &str, args: &[&str]) -> ExecutionResult {
    let output = Command::new(command)
        .args(args)
        .output()
        .map_err(|e| ExecutionResult::WithExecutionError(e.to_string()));

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            if output.status.success() {
                ExecutionResult::WithOutput(StdoutOutput {
                    stdout: (!stdout.is_empty()).then(move || stdout),
                    stderr: (!stderr.is_empty()).then(move || stderr),
                })
            } else {
                ExecutionResult::WithErrorCode(
                    output.status.code().unwrap_or(-1),
                    format!("Stdout: {}\nStderr: {}", stdout, stderr).into(),
                )
            }
        }
        Err(e) => e,
    }
}
