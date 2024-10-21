use iced::border::{radius, Radius};
use iced::widget::text::Shaping;
use iced::widget::text_editor::Style;
use iced::widget::{
    button, column, container, horizontal_rule, row, scrollable, text, text_editor, text_input,
    vertical_rule, Space,
};
use iced::{Alignment, Border, Color, Element, Length, Padding};
use uuid::Uuid;

pub enum State {
    SessionView,
    FriendView,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(text_editor::Action),
    MessageSend,
    SessionView,
    FriendView,
    SelectFriend(FriendItem),
    SwitchSession(FriendItem),
}

pub struct ChatView {
    pub state: State,
    pub new_message: text_editor::Content,
    pub messages: Vec<String>,
    pub friend_list: Vec<String>,
    pub selected_friend: Option<FriendItem>,
    pub session_list: Vec<String>,
    pub selected_session: Option<SessionItem>,
}

pub enum Action {
    None,
}

impl ChatView {
    pub fn new() -> Self {
        Self {
            state: State::FriendView,
            new_message: text_editor::Content::new(),
            messages: vec![],
            friend_list: vec![],
            selected_friend: None,
            session_list: vec![],
            selected_session: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Action {
        match message {
            Message::InputChanged(action) => {
                self.new_message.perform(action);
                Action::None
            }
            Message::MessageSend => {
                self.messages.push(self.new_message.text());
                self.new_message.perform(text_editor::Action::SelectAll);
                self.new_message
                    .perform(text_editor::Action::Edit(text_editor::Edit::Delete));
                println!("messages: {:#?}", self.new_message);
                Action::None
            }
            Message::SessionView => {
                self.state = State::SessionView;
                Action::None
            }
            Message::FriendView => {
                self.state = State::FriendView;
                Action::None
            }
            Message::SelectFriend(item) => {
                self.selected_friend = Some(item);
                Action::None
            }
            Message::SwitchSession(item) => {
                self.state = State::SessionView;
                let session = SessionItem {
                    sender_id: item.user_id,
                    receiver_id: Uuid::new_v4(),
                    receiver_name: String::from("receiver"),
                };
                println!("session: {:#?}", session);
                self.selected_session = Some(session);
                Action::None
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let nav_view = column![
            button("SE").on_press(Message::SessionView),
            button("FL").on_press(Message::FriendView)
        ]
        .width(Length::Fixed(40.))
        .spacing(20);

        let content_view = match self.state {
            State::SessionView => self.session_list_view(),
            State::FriendView => self.friend_list_view(),
        };

        row![nav_view, vertical_rule(1), content_view]
            .height(Length::Fill)
            .into()
    }

    fn session_list_view(&self) -> Element<Message> {
        let session_list = scrollable(column(
            self.session_list.iter().map(text).map(Element::from),
        ))
        .width(Length::Fixed(150.));

        let message_content = self
            .messages
            .iter()
            .map(|s| {
                container(text(s))
                    .style(|_| iced::widget::container::Style {
                        border: Border {
                            color: Color::BLACK,
                            width: 1.,
                            radius: radius(2.),
                        },
                        ..Default::default()
                    })
                    .max_width(200.)
            })
            .map(Element::from);

        let current_session = column![
            container(text("hello"))
                .height(Length::Fixed(20.))
                .align_y(Alignment::Center)
                .align_x(Alignment::Center),
            horizontal_rule(1),
            container(scrollable(column(message_content).spacing(10)).width(Length::Fill))
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(Padding {
                    top: 10.,
                    right: 2.,
                    bottom: 10.,
                    left: 10.
                }),
            horizontal_rule(1),
            container(scrollable(
                text_editor(&self.new_message)
                    .on_action(Message::InputChanged)
                    .style(|_, _| Style {
                        border: Border {
                            width: 0.,
                            color: Color::BLACK,
                            radius: radius(0)
                        },
                        background: iced::Background::Color(Color::from_rgba(1., 1., 1., 0.)),
                        icon: Color::BLACK,
                        placeholder: Color::BLACK,
                        value: Color::BLACK,
                        selection: Color::from_rgb(1., 0., 0.)
                    })
            ))
            .height(Length::Fixed(150.))
            .padding(10),
            container(row![
                Space::with_width(Length::Fill),
                button("send")
                    .height(30.)
                    .width(60.)
                    .on_press(Message::MessageSend)
            ])
            .padding([10, 20]),
        ]
        .width(Length::Fill);

        container(row![session_list, vertical_rule(1), current_session]).into()
    }

    fn friend_list_view(&self) -> Element<Message> {
        let friend_list = scrollable(column(
            self.friend_list
                .iter()
                .map(|name| {
                    button(text(name)).on_press(Message::SelectFriend(FriendItem {
                        user_id: Uuid::new_v4(),
                        username: name.clone(),
                    }))
                })
                .map(Element::from),
        ))
        .width(Length::Fixed(200.));

        let friend_info = if let Some(ref item) = self.selected_friend {
            container(row![
                Space::with_width(Length::FillPortion(1)),
                column![
                    text(item.user_id.to_string()),
                    text(item.username.clone()),
                    button(text("发送消息").shaping(Shaping::Advanced)).on_press(
                        Message::SwitchSession(FriendItem {
                            user_id: item.user_id.clone(),
                            username: item.username.clone()
                        })
                    )
                ]
                .spacing(10)
                .width(Length::FillPortion(8)),
                Space::with_width(Length::FillPortion(1)),
            ])
        } else {
            container(Space::with_width(Length::Fill))
        };

        container(row![friend_list, vertical_rule(1), friend_info]).into()
    }
}

#[derive(Debug, Clone)]
pub struct FriendItem {
    pub user_id: Uuid,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct SessionItem {
    pub sender_id: Uuid,
    pub receiver_id: Uuid,
    pub receiver_name: String,
}
