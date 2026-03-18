use std::{env, path::PathBuf};

use iced::{Color, Element, Task, widget::column};
use iced_layershell::{Settings, application, reexport::Anchor, settings::LayerShellSettings, to_layer_message};
use iced_moving_picture::gif;

fn main() {
    application(App::new, App::title, App::update, App::view)
        .style(App::style)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                size: Some((0, 70)),
                exclusive_zone: 0,
                keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::None,
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
        let path = PathBuf::from(env::var("HOME").unwrap()).join("pro/gif_sys/marija-nun.gif");
        if path.exists() {
            eprintln!("path");
        } else {
            eprintln!("path not found");
        }
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
                eprintln!("gif loaded");
                gif(&frames).into()
            }
            None => {
                eprintln!("gif not loaded");
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
