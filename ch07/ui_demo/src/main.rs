use iced::{button, executor, Application, Clipboard, Command, Element, Font, Settings, Text};

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular"),
};

fn main() -> iced::Result {
    GUI::run(Settings::default())
}

struct GUI {
    start_stop_button_state: button::State,
    reset_button_state: button::State,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    // fn new(_flags: ()) -> (Self, Command<Self::Message>) {
    //     (GUI, Command::none())
    // }
    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            GUI {
                start_stop_button_state: button::State::new(),
                reset_button_state: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(
        &mut self,
        message: Self::Message,
        clipboard: &mut Clipboard,
    ) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        let tick_text = Text::new("00:00:00.00").font(FONT).size(60);
    }
}
