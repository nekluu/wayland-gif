use std::path::PathBuf;

use iced::{Color, Element, Task, widget::column};
use iced_layershell::to_layer_message;
use iced_moving_picture::gif;

fn main() {
    println!("Hello, world!");
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
    fn title(&self) -> String {
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
