// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: MPL-2.0

//! Application API example

use cosmic::app::{Core, Settings, Task};
use cosmic::iced_core::Size;
use cosmic::widget::menu;
use cosmic::{executor, iced, ApplicationExt, Element};
use std::collections::HashMap;

/// Runs application with these settings
#[rustfmt::skip]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let _ = tracing_log::LogTracer::init();

    let settings = Settings::default()
        .size(Size::new(1024., 768.));

    cosmic::app::run::<App>(settings, ())?;

    Ok(())
}

/// Messages that are used specifically by our [`App`].
#[derive(Clone, Debug)]
pub enum Message {
    Clicked,
    ShowContext,
    WindowClose,
    ShowWindowMenu,
    ToggleHideContent,
    WindowNew,
}

/// The [`App`] stores application-specific state.
pub struct App {
    core: Core,
    button_label: String,
    show_context: bool,
    hide_content: bool,
}

/// Implement [`cosmic::Application`] to integrate with COSMIC.
impl cosmic::Application for App {
    /// Default async executor to use with the app.
    type Executor = executor::Default;

    /// Argument received [`cosmic::Application::new`].
    type Flags = ();

    /// Message type specific to our [`App`].
    type Message = Message;

    /// The unique application ID to supply to the window manager.
    const APP_ID: &'static str = "org.cosmic.ContextMenuDemo";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// Creates the application, and optionally emits command on initialize.
    fn init(core: Core, _input: Self::Flags) -> (Self, Task<Self::Message>) {
        let mut app = App {
            core,
            button_label: String::from("Right click me"),
            hide_content: false,
            show_context: false,
        };

        app.set_header_title("COSMIC Context Menu Demo".into());
        let command = app.set_window_title("COSMIC Context Menu Demo".into());

        (app, command)
    }

    /// Handle application events here.
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        self.button_label = format!("Clicked {message:?}");

        Task::none()
    }

    /// Creates a view after each update.
    fn view(&self) -> Element<Self::Message> {
        let widget = cosmic::widget::context_menu(
            cosmic::widget::button::text(&self.button_label).on_press(Message::Clicked),
            self.context_menu(),
        );

        let centered = cosmic::widget::container(widget)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .align_y(iced::alignment::Vertical::Center);

        Element::from(centered)
    }
}

impl App {
    fn context_menu(&self) -> Option<Vec<menu::Tree<Message>>> {
        Some(menu::items(
            &HashMap::new(),
            vec![
                menu::Item::Button("New window", ContextMenuAction::WindowNew),
                menu::Item::Divider,
                menu::Item::Folder(
                    "View",
                    vec![menu::Item::CheckBox(
                        "Hide content",
                        self.hide_content,
                        ContextMenuAction::ToggleHideContent,
                    )],
                ),
                menu::Item::Divider,
                menu::Item::Button("Quit", ContextMenuAction::WindowClose),
            ],
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContextMenuAction {
    WindowClose,
    ToggleHideContent,
    WindowNew,
}

impl menu::Action for ContextMenuAction {
    type Message = Message;
    fn message(&self) -> Self::Message {
        match self {
            ContextMenuAction::WindowClose => Message::WindowClose,
            ContextMenuAction::ToggleHideContent => Message::ToggleHideContent,
            ContextMenuAction::WindowNew => Message::WindowNew,
        }
    }
}
