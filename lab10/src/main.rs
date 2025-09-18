use iced::alignment::Horizontal;
use iced::widget::{button, column, container, row, text};
use iced::{Application, Command, Element, Length, Settings, Theme};

pub fn main() -> iced::Result {
    Calc::run(Settings::default())
}

struct Calc {
    display: String,         // shows the current entry/result, starts at "0"
    first: Option<f64>,      // LHS stored when an operator is chosen
    op: Option<Op>,          // pending operation (+ - * /)
    entering_new: bool,      // true when the next digit should start a new number
}

#[derive(Debug, Clone, Copy)]
enum Op { Add, Sub, Mul, Div }

#[derive(Debug, Clone)]
enum Message {
    Digit(u8),    // 0..=9
    SetOp(Op),    // + - * /
    Equals,       // =
    Clear,        // C
    Exit,         // X
}

impl Default for Calc {
    fn default() -> Self {
        Self {
            display: "0".into(),
            first: None,
            op: None,
            entering_new: false,
        }
    }
}

impl Application for Calc {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String { "Iced — Calculator (Complete)".into() }

    fn update(&mut self, msg: Self::Message) -> Command<Self::Message> {
        match msg {
            Message::Digit(n) => {
                if self.entering_new || self.display == "0" {
                    self.display = n.to_string();
                    self.entering_new = false;
                } else {
                    self.display.push(char::from(b'0' + n));
                }
            }

            Message::SetOp(new_op) => {
                let current = parse_f64(&self.display);
                match (self.first, self.op) {
                    (None, _) => {
                        self.first = Some(current);
                    }
                    (Some(lhs), Some(old_op)) if !self.entering_new => {
                        let result = apply(old_op, lhs, current);
                        self.display = result.to_string();
                        self.first = Some(result);
                    }
                    _ => {}
                }
                self.op = Some(new_op);
                self.entering_new = true;
            }

            Message::Equals => {
                if let (Some(lhs), Some(op)) = (self.first, self.op) {
                    let rhs = parse_f64(&self.display);
                    let result = apply(op, lhs, rhs);
                    self.display = result.to_string();
                    self.first = None;
                    self.op = None;
                    self.entering_new = true;
                }
            }

            Message::Clear => {
                *self = Self::default();
                self.display = "0".into();
            }

            Message::Exit => {
                std::process::exit(0);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        const BTN: f32 = 64.0;
        const GAP: f32 = 8.0;
        const GRID_W: f32 = BTN * 4.0 + GAP * 3.0;

        let btn = |label: &str, msg: Message| {
            button(text(label))
                .on_press(msg)
                .width(Length::Fixed(BTN))
                .height(Length::Fixed(BTN))
        };
        let digit = |n: u8| btn(&n.to_string(), Message::Digit(n));
        let op = |sym: &str, o: Op| btn(sym, Message::SetOp(o));

        let display = container(text(&self.display).size(36))
            .width(Length::Fixed(GRID_W))
            .padding([8, 12])
            .align_x(Horizontal::Right);

        let r1 = row![ digit(7), digit(8), digit(9), op("/", Op::Div) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));
        let r2 = row![ digit(4), digit(5), digit(6), op("*", Op::Mul) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));
        let r3 = row![ digit(1), digit(2), digit(3), op("-", Op::Sub) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));
        let r4 = row![ digit(0), btn("C", Message::Clear), btn("=", Message::Equals), op("+", Op::Add) ]
            .spacing(GAP).width(Length::Fixed(GRID_W));

        let exit = button(text("Exit (X)"))
            .on_press(Message::Exit)
            .width(Length::Fixed(GRID_W))
            .height(Length::Fixed(48.0));

        column![display, r1, r2, r3, r4, exit]
            .spacing(GAP)
            .padding(12)
            .into()
    }
}

/* ──────────────── Utilities ──────────────── */

fn parse_f64(s: &str) -> f64 {
    s.parse::<f64>().unwrap_or(0.0)
}

fn apply(op: Op, a: f64, b: f64) -> f64 {
    match op {
        Op::Add => a + b,
        Op::Sub => a - b,
        Op::Mul => a * b,
        Op::Div => {
            if b == 0.0 {
                0.0 // policy: divide-by-zero = 0
            } else {
                a / b
            }
        }
    }
}
