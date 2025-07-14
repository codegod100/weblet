// SPDX-License-Identifier: GPL-3.0-only

use cosmic::app::{Core, Task};
use cosmic::iced::window::Id;
use cosmic::iced::Limits;
use cosmic::iced_winit::commands::popup::{destroy_popup, get_popup};
use cosmic::widget::{self, settings, autosize};
use cosmic::{Application, Element};
use cosmic::applet::cosmic_panel_config::PanelAnchor;
use cosmic::widget::Id as WId;
use std::sync::LazyLock;
use crate::webview_widget;

static AUTOSIZE_MAIN_ID: LazyLock<WId> = std::sync::LazyLock::new(|| WId::new("autosize-main"));

use crate::fl;

/// This is the struct that represents your application.
/// It is used to define the data that will be used by your application.
#[derive(Default)]
pub struct YourApp {
    /// Application state which is managed by the COSMIC runtime.
    core: Core,
    /// The popup id.
    popup: Option<Id>,
    /// Example row toggler.
    example_row: bool,
}

/// This is the enum that contains all the possible variants that your application will need to transmit messages.
/// This is used to communicate between the different parts of your application.
/// If your application does not need to send messages, you can use an empty enum or `()`.
#[derive(Debug, Clone)]
pub enum Message {
    TogglePopup,
    PopupClosed(Id),
    ToggleExampleRow(bool),
}

/// Implement the `Application` trait for your application.
/// This is where you define the behavior of your application.
///
/// The `Application` trait requires you to define the following types and constants:
/// - `Executor` is the async executor that will be used to run your application's commands.
/// - `Flags` is the data that your application needs to use before it starts.
/// - `Message` is the enum that contains all the possible variants that your application will need to transmit messages.
/// - `APP_ID` is the unique identifier of your application.
impl Application for YourApp {
    type Executor = cosmic::executor::Default;

    type Flags = ();

    type Message = Message;

    const APP_ID: &'static str = "com.example.CosmicAppletTemplate";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    /// This is the entry point of your application, it is where you initialize your application.
    ///
    /// Any work that needs to be done before the application starts should be done here.
    ///
    /// - `core` is used to passed on for you by libcosmic to use in the core of your own application.
    /// - `flags` is used to pass in any data that your application needs to use before it starts.
    /// - `Command` type is used to send messages to your application. `Command::none()` can be used to send no messages to your application.
    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = YourApp {
            core,
            ..Default::default()
        };

        (app, Task::none())
    }

    fn on_close_requested(&self, id: Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    /// This is the main view of your application, it is the root of your widget tree.
    ///
    /// The `Element` type is used to represent the visual elements of your application,
    /// it has a `Message` associated with it, which dictates what type of message it can send.
    ///
    /// To get a better sense of which widgets are available, check out the `widget` module.
    fn view(&self) -> Element<Self::Message> {
        // Create a row of elements like the monitor applet does
        let webview_content = Element::new(
            webview_widget::webview::<Self::Message>("data:text/html,<html><body style='margin:0;padding:4px;background:#333;color:white;font-family:sans-serif;font-size:11px;'>CPU: 45%</body></html>")
                .width(cosmic::iced::Length::Fill)
                .height(cosmic::iced::Length::Fill)
        );

        let webview_content2 = Element::new(
            webview_widget::webview::<Self::Message>("data:text/html,<html><body style='margin:0;padding:4px;background:#333;color:white;font-family:sans-serif;font-size:11px;'>RAM: 2.1GB</body></html>")
                .width(cosmic::iced::Length::Fill)
                .height(cosmic::iced::Length::Fill)
        );

        let webview_content3 = Element::new(
            webview_widget::webview::<Self::Message>("data:text/html,<html><body style='margin:0;padding:4px;background:#333;color:white;font-family:sans-serif;font-size:11px;'>↓125MB/s</body></html>")
                .width(cosmic::iced::Length::Fill)
                .height(cosmic::iced::Length::Fill)
        );

        // Get suggested padding and spacing
        let horizontal = matches!(
            self.core.applet.anchor,
            PanelAnchor::Top | PanelAnchor::Bottom
        );

        let theme = cosmic::theme::active();
        let spacing = theme.cosmic().space_xs();

        // Create a row of widgets like monitor applet
        let elements: Element<Self::Message> = if horizontal {
            cosmic::iced::widget::Row::new()
                .push(webview_content)
                .push(webview_content2)
                .push(webview_content3)
                .spacing(spacing)
                .align_y(cosmic::iced::Alignment::Center)
                .into()
        } else {
            cosmic::iced::widget::Column::new()
                .push(webview_content)
                .push(webview_content2)
                .push(webview_content3)
                .spacing(spacing)
                .align_x(cosmic::iced::Alignment::Center)
                .into()
        };

        // Create a button wrapper
        let button = widget::button::custom(elements)
            .padding(if horizontal {
                [0, self.core.applet.suggested_padding(true)]
            } else {
                [self.core.applet.suggested_padding(true), 0]
            })
            .class(cosmic::theme::Button::AppletIcon)
            .on_press(Message::TogglePopup);

              let mut limits = Limits::NONE.min_width(1.).min_height(1.);
        if let Some(b) = self.core.applet.suggested_bounds {
            if b.width > 0.0 {
                limits = limits.max_width(b.width);
            }
            if b.height > 0.0 {
                limits = limits.max_height(b.height);
            }
        }
        // Use autosize with safe limits
         let limits = Limits::NONE
                .max_width(420.0)
                .min_width(360.0)
                .min_height(200.0)
                .max_height(600.0);

        autosize::autosize(widget::container(button), AUTOSIZE_MAIN_ID.clone())
            .limits(limits)
            .into()
    }

    fn view_window(&self, _id: Id) -> Element<Self::Message> {
        let content_list = widget::list_column()
            .padding(5)
            .spacing(0)
            .add(settings::item(
                fl!("example-row"),
                widget::toggler(self.example_row).on_toggle(Message::ToggleExampleRow),
            ))
            .add(widget::text("WebView Placeholder (https://example.com)"));

        self.core.applet.popup_container(content_list).into()
    }

    /// Application messages are handled here. The application state can be modified based on
    /// what message was received. Commands may be returned for asynchronous execution on a
    /// background thread managed by the application's executor.
    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::TogglePopup => {
                return if let Some(p) = self.popup.take() {
                    destroy_popup(p)
                } else {
                    let new_id = Id::unique();
                    self.popup.replace(new_id);
                    let mut popup_settings = self.core.applet.get_popup_settings(
                        self.core.main_window_id().unwrap(),
                        new_id,
                        None,
                        None,
                        None,
                    );
                    popup_settings.positioner.size_limits = Limits::NONE
                        .max_width(372.0)
                        .min_width(300.0)
                        .min_height(200.0)
                        .max_height(1080.0);
                    get_popup(popup_settings)
                }
            }
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::ToggleExampleRow(toggled) => self.example_row = toggled,
        }
        Task::none()
    }

    fn style(&self) -> Option<cosmic::iced_runtime::Appearance> {
        Some(cosmic::applet::style())
    }
}
