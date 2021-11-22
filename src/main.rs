extern crate repng;
extern crate scrap;

use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use iced::{Align, button, Button, Column, Container, Element, Image, Length, Sandbox, Settings, Text};
use scrap::{Capturer, Display};

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
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
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let img =
            Image::new("screenshot.png")
                .width(Length::Fill)
                .height(Length::Fill);


        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()).size(50))
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                    .on_press(Message::DecrementPressed),
            )
            .push(img)
            .into()
    }
}

fn main() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    let mut bitflipped = Vec::with_capacity(w * h * 4);
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

        // Flip the ARGB image into a BGRA image.

        let stride = buffer.len() / h;

        for y in 0..h {
            for x in 0..w {
                let i = stride * y + 4 * x;
                bitflipped.extend_from_slice(&[
                    buffer[i + 2],
                    buffer[i + 1],
                    buffer[i],
                    255,
                ]);
            }
        }


        repng::encode(
            File::create("screenshot.png").unwrap(),
            w as u32,
            h as u32,
            &bitflipped,
        ).unwrap();

        println!("Image saved to `screenshot.png`.");
        break;
    }


    Counter::run(Settings::default());
}
