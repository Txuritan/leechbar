extern crate image;
extern crate leechbar;

use leechbar::component::{Component, ComponentPosition};
use leechbar::BarBuilder;
use std::time::Duration;
use std::thread;

struct ImageComponent {
    index: u32,
    index_reset: u32,
    ordinal: u32,
    timeout: u64,
}

impl Component for ImageComponent {
    fn background(&mut self) -> Option<image::DynamicImage> {
        let name = format!("./testimages/image{}.png", self.index);
        let image = image::open(&name).unwrap();

        self.index += 2;
        if self.index > 9 {
            self.index = self.index_reset;
        }
        Some(image)
    }

    fn position(&mut self) -> ComponentPosition {
        ComponentPosition::LEFT(self.ordinal)
    }

    fn text(&mut self) -> Option<String> {
        None
    }

    fn timeout(&mut self) -> Duration {
        Duration::from_millis(self.timeout)
    }

    fn event(&mut self) {}
}

fn main() {
    let image = image::open("img.png").unwrap();
    let mut bar = BarBuilder::new()
        .background_color(255, 0, 255, 255)
        .foreground_color(0, 255, 0, 255)
        .background_image(image)
        .height(30)
        .output("DVI-1")
        .spawn()
        .unwrap();

    bar.draw(ImageComponent {
        index: 0,
        index_reset: 0,
        ordinal: 0,
        timeout: 5,
    });
    bar.draw(ImageComponent {
        index: 1,
        index_reset: 1,
        ordinal: 1,
        timeout: 3,
    });

    loop {
        thread::sleep(Duration::from_secs(999_999));
    }
}
