use iced::{Application, Command, Element, Settings, executor, widget::{button, column, text}};
use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader, sync::Arc};

#[derive(Debug, Clone)]
enum Message {
    PlayPressed,
}

struct Olive {
    audio_sink: Option<Arc<Sink>>,
}

impl Application for Olive {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self { audio_sink: None },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("olive - a music player")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PlayPressed => {
                if self.audio_sink.is_none() {
                    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                    let sink = Sink::try_new(&stream_handle).unwrap();

                    let file = BufReader::new(File::open("sample.mp3").unwrap());
                    let source = Decoder::new(file).unwrap();
                    sink.append(source);

                    self.audio_sink = Some(Arc::new(sink));
                }
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("olive - a music player"),
            button("Play music").on_press(Message::PlayPressed),
        ]
        .spacing(20)
        .padding(40);

        content.into()
    }
}

fn main() -> iced::Result {
    Olive::run(Settings::default())
}
