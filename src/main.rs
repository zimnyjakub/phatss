extern crate repng;
extern crate scrap;

use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use iced::{Align, button, Button, Column, Container, Element, Image, Length, Sandbox, Settings, Text};
use iced::image::Handle;
use scrap::{Capturer, Display};

#[derive(Default)]
struct Counter {
    value: i32,
    capture_button: button::State,
    image_width: u32,
    image_height: u32,
    image_data: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    CaptureImagePressed
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::CaptureImagePressed => {
                let one_second = Duration::new(1, 0);
                let one_frame = one_second / 60;

                let display = Display::primary().expect("Couldn't find primary display.");
                let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
                let (w, h) = (capturer.width(), capturer.height());
                self.image_width = w as u32;
                self.image_height = h as u32;

                loop {
                    // Wait until there's a frame.

                    let buffer = match capturer.frame() {
                        Ok(buffer) => buffer,
                        Err(error) => {
                            if error.kind() == WouldBlock {
                                // Keep spinning.
                                thread::sleep(one_frame);
                                continue;
                            } else {
                                panic!("Error: {}", error);
                            }
                        }
                    };

                    println!("Captured! Saving...");


                    self.image_data.clear();
                    self.image_data = buffer.to_vec();


                    break;
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let img =
            Image::new(Handle::from_pixels(self.image_width, self.image_height, (*self.image_data).to_vec()))
                .width(Length::Fill)
                .height(Length::Fill);

        Column::new()
            .padding(20)
            .align_items(Align::Start)
            .push(
                Button::new(&mut self.capture_button, Text::new("capture"))
                    .on_press(Message::CaptureImagePressed)
            )
            .push(img)
            .into()
    }
}

fn main() {
    Counter::run(Settings::default());
}
