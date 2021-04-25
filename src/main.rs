use iced::{button, Align, Button, Column, Row, Element, Sandbox, Settings, Text, Length};
const WINDOW_SIZE_HEIGHT: u32 = 500;
const WINDOW_SIZE_WIDTH: u32 = 300;
const WINDOW_IS_RESIZABLE: bool = false;
const WINDOW_TITLE: &str = "Simple calculator";

#[derive(Debug, Clone, Copy)]
enum ScOperator {
    Minus,
    Plus,
}

#[derive(Default)]
struct ScApp {
    /// The calculator value
    value: isize,
    operator: Option<ScOperator>,
    /// Buttons
    zero_button: button::State,
    one_button: button::State,
    two_button: button::State,
    three_button: button::State,
    four_button: button::State,
    five_button: button::State,
    six_button: button::State,
    seven_button: button::State,
    eight_button: button::State,
    nine_button: button::State,
    minus_button: button::State,
    plus_button: button::State,
    reset_button: button::State,
    equal_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum ScMessage {
    ZeroButtonPressed,
    OneButtonPressed,
    TwoButtonPressed,
    ThreeButtonPressed,
    FourButtonPressed,
    FiveButtonPressed,
    SixButtonPressed,
    SevenButtonPressed,
    EightButtonPressed,
    NineButtonPressed,
    MinusButtonPressed,
    PlusButtonPressed,
    ResetButtonPressed,
    EqualButtonPressed,
}

impl Sandbox for ScApp {
    type Message = ScMessage;

    fn new() -> Self {
        let mut app = Self::default();
        app.operator = Some(ScOperator::Plus);
        app
    }

    fn title(&self) -> String {
        String::from(WINDOW_TITLE)
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            ScMessage::ZeroButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 0;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 0;
                }
            }
            ScMessage::OneButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 1;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 1;
                }
            }
            ScMessage::TwoButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 2;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 2;
                }
            }
            ScMessage::ThreeButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 3;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 3;
                }
            }
            ScMessage::FourButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 4;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 4;
                }
            }
            ScMessage::FiveButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 5;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 5;
                }
            }
            ScMessage::SixButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 6;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 6;
                }
            }
            ScMessage::SevenButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 7;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 7;
                }
            }
            ScMessage::EightButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 8;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 8;
                }
            }
            ScMessage::NineButtonPressed => {
                if let ScOperator::Plus = self.operator.unwrap() {
                    self.value += 9;
                } else if let ScOperator::Minus = self.operator.unwrap() {
                    self.value -= 9;
                }
            }
            ScMessage::MinusButtonPressed => {
                self.operator = Some(ScOperator::Minus);
            }
            ScMessage::PlusButtonPressed => {
                self.operator = Some(ScOperator::Plus);
            }
            ScMessage::ResetButtonPressed => {
                self.value = 0;
                self.operator = Some(ScOperator::Plus);
            }
            ScMessage::EqualButtonPressed => {
                println!("Result : {}", self.value);
            }
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(20)
            .align_items(Align::Center)
            .push(
                Row::new()
                    .push(Text::new(self.value.to_string()).size(50))
            )
            .push(
                Row::with_children(vec![
                    Element::from(Button::new(&mut self.zero_button, Text::new("0")).on_press(ScMessage::ZeroButtonPressed)),
                    Element::from(Button::new(&mut self.one_button, Text::new("1")).on_press(ScMessage::OneButtonPressed)),
                    Element::from(Button::new(&mut self.two_button, Text::new("2")).on_press(ScMessage::TwoButtonPressed)),
                    Element::from(Button::new(&mut self.three_button, Text::new("3")).on_press(ScMessage::ThreeButtonPressed)),
                    Element::from(Button::new(&mut self.four_button, Text::new("4")).on_press(ScMessage::FourButtonPressed)),
                ])
            )
            .push(
                Row::with_children(vec![
                    Element::from(Button::new(&mut self.five_button, Text::new("5")).on_press(ScMessage::FiveButtonPressed)),
                    Element::from(Button::new(&mut self.six_button, Text::new("6")).on_press(ScMessage::SixButtonPressed)),
                    Element::from(Button::new(&mut self.seven_button, Text::new("7")).on_press(ScMessage::SevenButtonPressed)),
                    Element::from(Button::new(&mut self.eight_button, Text::new("8")).on_press(ScMessage::EightButtonPressed)),
                    Element::from(Button::new(&mut self.nine_button, Text::new("9")).on_press(ScMessage::NineButtonPressed)),
                ])
            )
            .push(
                Row::with_children(vec![
                    Element::from(
                        Button::new(&mut self.plus_button, Text::new("+")).on_press(ScMessage::PlusButtonPressed)),
                    Element::from(Button::new(&mut self.minus_button, Text::new("-")).on_press(ScMessage::MinusButtonPressed)),
                    Element::from(Button::new(&mut self.equal_button, Text::new("=")).on_press(ScMessage::EqualButtonPressed)),
                ])
            )
            .into()
    }
}

fn main() -> iced::Result {
    let mut app_settings = Settings::default();
    app_settings.window.resizable = WINDOW_IS_RESIZABLE;
    app_settings.window.size = (WINDOW_SIZE_WIDTH, WINDOW_SIZE_HEIGHT);

    ScApp::run(app_settings)
}