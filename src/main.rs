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
struct ScButtons {
    zero: button::State,
    one: button::State,
    two: button::State,
    three: button::State,
    four: button::State,
    five: button::State,
    six: button::State,
    seven: button::State,
    eight: button::State,
    nine: button::State,
    minus: button::State,
    plus: button::State,
    reset: button::State,
    equal: button::State,
}

#[derive(Default)]
struct ScApp {
    state: ScState,
    value: isize,
    /// The calculator result
    result: isize,
    /// Operator type
    operator: Option<ScOperator>,
    /// Buttons
    buttons: ScButtons
}

#[derive(Default)]
struct ScState {
    result: isize,
    cache: (Option<isize>, Option<ScOperator>, Option<isize>)
}

impl ScState {
    fn new() -> Self {
        ScState { result: 0, cache: (None, None, None) }
    }

    fn get_operator(&self) -> Option<ScOperator> {
        self.cache.1
    }
    fn get_first(&self) -> Option<isize> {
        self.cache.0
    }
    fn get_last(&self) -> Option<isize> {
        self.cache.2
    }

    fn set_operator(&mut self, op: Option<ScOperator>) {
        self.cache.1 = op;
    }
    fn set_first(&mut self, number: Option<isize>) {
        self.cache.0 = number;
    }
    fn set_last(&mut self, number: Option<isize>) {
        self.cache.2 = number;
    }

    fn result(&mut self) {
        match self.cache.1 {
            None => {}
            Some(operator) => {
                match operator {
                    ScOperator::Minus => {
                        self.result = self.cache.0.unwrap() - self.cache.2.unwrap();
                    }
                    ScOperator::Plus => {
                        self.result =  self.cache.0.unwrap() + self.cache.2.unwrap();
                    }
                }
            }
        }
    }

    fn reset(&mut self) {
        self.set_first(Some(self.result));
        self.set_operator(None);
        self.set_last(None);
    }
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

impl ScApp {
    fn add_value(&mut self, value: isize) {
        match self.state.get_last() {
            Some(_) => {
                match self.state.get_last() {
                    Some(v) => {
                        let value = Some(format!("{}{}", v, value).parse().unwrap());
                        self.state.set_last(value);
                        self.value = value.unwrap();
                    }
                    None => {
                        let value = Some(value);
                        self.state.set_last(value);
                        self.value = value.unwrap();
                    }
                }
            }
            None => {
                match self.state.get_operator() {
                    None => {
                        match self.state.get_first() {
                            Some(v) => {
                                let value = Some(format!("{}{}", v, value).parse().unwrap());
                                self.state.set_first(value);
                                self.value = value.unwrap();
                            }
                            None => {
                                let value = Some(value);
                                self.state.set_first(value);
                                self.value = value.unwrap();
                            }
                        }
                    }
                    Some(_) => {
                        let value = Some(value);
                        self.state.set_last(value);
                        self.value = value.unwrap();
                    }
                }
            }
        }
    }

    fn set_operator(&mut self, op: Option<ScOperator>) {
        match self.state.get_operator() {
            None => {
                self.state.set_operator(op);
            }
            Some(_) => {}
        }
    }
}

impl Sandbox for ScApp {
    type Message = ScMessage;

    fn new() -> Self {
        let mut app = Self::default();
        app.operator = Some(ScOperator::Plus);
        app.state = ScState::new();
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
                self.add_value(1);
            }
            ScMessage::TwoButtonPressed => {
                self.add_value(2);
            }
            ScMessage::ThreeButtonPressed => {
                self.add_value(3);
            }
            ScMessage::FourButtonPressed => {
                self.add_value(4);
            }
            ScMessage::FiveButtonPressed => {
                self.add_value(5);
            }
            ScMessage::SixButtonPressed => {
                self.add_value(6);
            }
            ScMessage::SevenButtonPressed => {
                self.add_value(7);
            }
            ScMessage::EightButtonPressed => {
                self.add_value(8);
            }
            ScMessage::NineButtonPressed => {
                self.add_value(9);
            }
            ScMessage::MinusButtonPressed => {
                self.set_operator(Some(ScOperator::Minus))
            }
            ScMessage::PlusButtonPressed => {
                self.set_operator(Some(ScOperator::Plus))
            }
            ScMessage::ResetButtonPressed => {}
            ScMessage::EqualButtonPressed => {
                self.state.result();
                self.value = self.state.result;
                self.state.reset();
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
                    Element::from(Button::new(&mut self.buttons.zero, Text::new("0")).on_press(ScMessage::ZeroButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.one, Text::new("1")).on_press(ScMessage::OneButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.two, Text::new("2")).on_press(ScMessage::TwoButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.three, Text::new("3")).on_press(ScMessage::ThreeButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.four, Text::new("4")).on_press(ScMessage::FourButtonPressed)),
                ])
            )
            .push(
                Row::with_children(vec![
                    Element::from(Button::new(&mut self.buttons.five, Text::new("5")).on_press(ScMessage::FiveButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.six, Text::new("6")).on_press(ScMessage::SixButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.seven, Text::new("7")).on_press(ScMessage::SevenButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.eight, Text::new("8")).on_press(ScMessage::EightButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.nine, Text::new("9")).on_press(ScMessage::NineButtonPressed)),
                ])
            )
            .push(
                Row::with_children(vec![
                    Element::from(
                        Button::new(&mut self.buttons.plus, Text::new("+")).on_press(ScMessage::PlusButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.minus, Text::new("-")).on_press(ScMessage::MinusButtonPressed)),
                    Element::from(Button::new(&mut self.buttons.equal, Text::new("=")).on_press(ScMessage::EqualButtonPressed)),
                ])
            )
            .push(
                Row::new()
                    .push(Text::new(self.state.result.to_string()).size(50))
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