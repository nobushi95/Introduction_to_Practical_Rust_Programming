use iced::{
    button, executor, Align, Application, Button, Clipboard, Column, Command, Element, Font,
    Length, Row, Settings, Text,
};

const FONT: Font = Font::External {
    name: "PixelMplus12-Regular",
    bytes: include_bytes!("../rsc/PixelMplus12-Regular.ttf"),
};

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400u32, 120u32);
    GUI::run(settings);
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
        let start_stop_button = Button::new(
            &mut self.start_stop_button_state,
            Text::new("Start")
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .font(FONT),
        )
        .min_width(80);

        let reset_button = Button::new(
            &mut self.reset_button_state,
            Text::new("Reset")
                .horizontal_alignment(iced::HorizontalAlignment::Center)
                .font(FONT),
        );

        Column::new()
            .push(tick_text)
            .push(
                Row::new()
                    .push(start_stop_button)
                    .push(reset_button)
                    .spacing(10),
            )
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}
