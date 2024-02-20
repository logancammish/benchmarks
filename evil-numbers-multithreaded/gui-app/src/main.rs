use std::time::SystemTime;
use iced::window::{icon, Icon};
use iced::widget::{button, column, slider, text,text_input, vertical_space };
use iced::{Alignment, Element, Sandbox};
use image;
use std::thread;
use num_cpus;

pub fn main() -> iced::Result {
    Counter::run(iced::Settings{ 
        antialiasing: true,
        window: iced::window::Settings {
            size: (500, 400), 
            .. iced::window::Settings::default()
        },

        ..iced::Settings::default() 
    })
}

struct Counter {
    tests: i32,
    length: i32,
    text: String
}

#[derive(Debug, Clone)]
enum Message {
    TestBegun,
    Tests(i32),
    Length(i32),    
    TestsString(String),
    LengthString(String)
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { text: "null".to_string(), tests: 10, length: 100000 }
    }

    fn title(&self) -> String {
        String::from("Evil Numbers benchmark")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::TestBegun => {
                let numcpus = num_cpus::get();
                println!("{numcpus}");

                let mut time: i32 = 0;
                for _ in 1..(self.tests + 1) {
                    let length = self.length;
                    let start = SystemTime::now();
                    
                    for _ in 1..numcpus {
                        let handle = thread::spawn(move || {
                            thread::spawn(move || { 
                                for i in 1..length {
                                    if format!("{:b}",i).matches("1").count() % 2 == 0 {}
                                }
                            });
                        });
                        handle.join().unwrap();
                    }

                    let finish = SystemTime::now();
                    let duration = finish
                        .duration_since(start)
                        .unwrap()
                        .as_millis() as i32;
                    time += duration;
                }
                
                self.text = format!("Average time: {} ms | Overall time: {} ms (≈{} s)\nScore: {}", 
                    time / (self.tests + 1), time, time / 1000,
                    (time ^ 2) * (self.length / 10000) / self.tests );
            }
            Message::Tests(tests) => { 
                self.tests = tests;
                self.text = format!("Tests: {}\nLength: {}", self.tests, self.length).to_string(); 
            }
            Message::Length(length) => { 
                self.length = length;
                self.text = format!("Tests: {}\nLength: {}", self.tests, self.length).to_string(); 
            }
            Message::TestsString(text) => {
                self.tests = text.parse::<i32>().unwrap();
            }
            Message::LengthString(text) => {
                self.length = text.parse::<i32>().unwrap();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text("Tests:").size(30),
            slider(1..=500, self.tests, Message::Tests),
            text_input("Tests", &self.tests.to_string())
                .on_input(Message::TestsString)
                .on_submit(Message::Tests(self.tests))
                .padding(10),

            text("Length:").size(30),
            slider(100000..=5000000, self.length, Message::Length),
            text_input("Length", &self.length.to_string())
                .on_input(Message::LengthString)
                .on_submit(Message::Tests(self.length))
                .padding(10),
            vertical_space(5),

            button("begin")
                .on_press(Message::TestBegun)
                .padding(5),
            
            text(&self.text)
                .size(30),
            vertical_space(15),
            text("Licensed under the GPL-3.0 license, created by Logan Cammish 2024.")
                .size(10)
        ].padding(20)
        .align_items(Alignment::Start)
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::default()
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}
