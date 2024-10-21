use std::collections::HashMap;

use chat::ChatView;
use iced::{widget::text_editor, window, Element, Task};
use login::LoginView;
use nexus_core::model::LoginResponse;

pub mod chat;
pub mod login;

fn main() -> iced::Result {
    iced::application("Nexus", update, view)
        // .window_size([720., 480.])
        .window(window::Settings {
            size: [720., 480.].into(),
            min_size: Some([640., 400.].into()),
            ..Default::default()
        })
        .run()
}

struct State {
    screen: Screen,
}

impl Default for State {
    fn default() -> Self {
        Self {
            screen: Screen::LoginView(LoginView::new()),
        }
    }
}

enum Screen {
    LoginView(LoginView),
    ChatView(ChatView),
}

#[derive(Debug, Clone)]
enum Message {
    LoginView(login::Message),
    ChatView(chat::Message),
    LoginResponse(bool),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::LoginView(message) => {
            if let Screen::LoginView(view) = &mut state.screen {
                let action = view.update(message);

                match action {
                    login::Action::None => Task::none(),
                    login::Action::Login(username, password) => {
                        Task::perform(login_request(username, password), Message::LoginResponse)
                    }
                }
            } else {
                Task::none()
            }
        }
        Message::ChatView(message) => {
            if let Screen::ChatView(view) = &mut state.screen {
                let action = view.update(message);
                match action {
                    chat::Action::None => Task::none(),
                }
            } else {
                Task::none()
            }
        }
        Message::LoginResponse(resp) => {
            if resp {
                state.screen = Screen::ChatView(ChatView {
                    new_message: text_editor::Content::new(),
                    messages: vec![
                        "hello from client1".to_string(),
                        "resp from client2".to_string(),
                    ],
                    state: chat::State::FriendView,
                    friend_list: vec![
                        "Jack".to_string(),
                        "Rose".to_string(),
                        "June".to_string(),
                        "White".to_string(),
                    ],
                    selected_friend: None,
                    session_list: vec![],
                    selected_session: None,
                });
                Task::none()
            } else {
                Task::none()
            }
        }
    }
}

fn view(state: &State) -> Element<Message> {
    match &state.screen {
        Screen::LoginView(login_view) => login_view.view().map(Message::LoginView),
        Screen::ChatView(chat_view) => chat_view.view().map(Message::ChatView),
    }
}

async fn login_request(username: String, password: String) -> bool {
    let mut map = HashMap::new();
    map.insert("username", &username);
    map.insert("password", &password);

    let client = reqwest::Client::new();
    match client
        .post("http://127.0.0.1:9000/login")
        .json(&map)
        .send()
        .await
    {
        Ok(msg) => {
            let resp = msg.json::<LoginResponse>().await.unwrap();
            resp.is_success
        }
        Err(_) => false,
    }
}
