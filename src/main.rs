use std::path::PathBuf;

use iced::{Color, Element, Task, widget::column};
use iced_layershell::{Settings, application, reexport::Anchor, settings::LayerShellSettings, to_layer_message};
use iced_moving_picture::gif;

fn main() {
    application(App::default, App::title, App::update, App::view)
        .style(App::style)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: Some((0, 400)),
                exclusive_zone: 400,
                anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
                start_mode: iced_layershell::settings::StartMode::Active,
                ..Default::default()
            },
            ..Default::default()
        })
        .run().unwrap();
}

#[to_layer_message]
#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<gif::Frames,gif::Error>),
}
#[derive(Debug,Default)]
struct App {
    frames: Option<gif::Frames>,
}  
impl App {
    fn new() -> (Self,Task<Message>) {
        let path = PathBuf::from("home/neglu/pro/gif_sys/marija-nun.gif");
        (
            App::default(),
            gif::Frames::load_from_path(path).map(Message::Loaded),
        )
    }
    fn title() -> String {
        "Wayland GIF".to_owned()
    }
    fn update(&mut self, message: Message) -> Task<Message>{
        match message {
            Message::Loaded(result) => {
                self.frames = result.ok();
                Task::none()
            }
            _ => unreachable!()
        }
    }
    fn view(&self) -> Element<'_,Message> {
        match &self.frames {
            Some(frames) => {
                gif(&frames).into()
            }
            None => {
                column![].into()
            }
        }
    }
    fn style(&self, theme: &iced::Theme) -> iced::theme::Style {
        use iced::theme::Style;
        Style {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
    }
