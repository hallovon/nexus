use iced::widget::{button, column, text_input};
use iced::Element;

#[derive(Debug, Clone)]
pub enum Message {
    UsernameChanged(String),
    PasswordChanged(String),
    LoginButtonPressed,
}

pub struct LoginView {
    username: String,
    password: String,
}

pub enum Action {
    Login(String, String),
    None,
}

impl LoginView {
    pub fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::UsernameChanged(s) => {
                self.username = s;
                Action::None
            }
            Message::PasswordChanged(s) => {
                self.password = s;
                Action::None
            }
            Message::LoginButtonPressed => {
                let username = self.username.clone();
                let password = self.password.clone();
                self.password.clear();
                Action::Login(username, password)
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        column![
            text_input("username", &self.username).on_input(Message::UsernameChanged),
            text_input("password", &self.password).on_input(Message::PasswordChanged),
            button("login").on_press(Message::LoginButtonPressed)
        ]
        .spacing(10)
        .into()
    }
}
