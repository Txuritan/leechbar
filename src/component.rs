use image::DynamicImage;
use std::time::Duration;
use builder;

// Alignment of component or text
#[derive(Clone, Copy)]
pub enum Alignment {
    LEFT,
    CENTER,
    RIGHT,
}

impl Alignment {
    // Calculate the x-offset of a component based on its alignment
    pub fn x_offset(&self, comp_width: u16, width: u16) -> i16 {
        match *self {
            Alignment::LEFT => 0,
            Alignment::CENTER => (f64::from(comp_width) / 2. - f64::from(width) / 2.) as i16,
            Alignment::RIGHT => (comp_width - width) as i16,
        }
    }
}

// Alignment and position of a component
#[derive(Clone, Copy)]
pub struct ComponentPosition {
    alignment: Alignment,
    ordinal: u32,
}

impl ComponentPosition {
    // Create a new component position
    pub fn new(alignment: Alignment, ordinal: u32) -> Self {
        ComponentPosition { alignment, ordinal }
    }

    // Use the position and alignment of the item to get a unique id
    pub fn unique_id(&self) -> u32 {
        match self.alignment {
            Alignment::LEFT => self.ordinal * 3,
            Alignment::CENTER => self.ordinal * 3 + 1,
            Alignment::RIGHT => self.ordinal * 3 + 2,
        }
    }
}

// Struct for a text element
pub struct Text {
    pub content: String,
    pub font: Option<String>,
    pub color: Option<(f64, f64, f64, f64)>,
    pub alignment: Alignment,
}

impl Text {
    // Create a text element
    pub fn new<T: Into<String>>(content: T) -> Self {
        Text {
            content: content.into(),
            font: None,
            color: None,
            alignment: Alignment::CENTER,
        }
    }

    // Set the font of the text element
    pub fn font<T: Into<String>>(mut self, font: T) -> Self {
        self.font = Some(font.into());
        self
    }

    // Set the foreground color of the text
    pub fn color(mut self, red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        self.color = Some((
            f64::from(red) / 255.,
            f64::from(green) / 255.,
            f64::from(blue) / 255.,
            f64::from(alpha) / 255.,
        ));
        self
    }

    // Set the alignment of the text element
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

// Struct for an image element
pub struct Background {
    pub color: Option<u32>,
    pub image: Option<DynamicImage>,
    pub alignment: Alignment,
    pub min_width: u16,
}

impl Background {
    // Create an image element
    pub fn new_image(image: DynamicImage) -> Self {
        Background {
            image: Some(image),
            color: None,
            alignment: Alignment::CENTER,
            min_width: 0,
        }
    }

    // Create a color element
    pub fn new_color(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Background {
            image: None,
            color: Some(builder::color(red, green, blue, alpha)),
            alignment: Alignment::CENTER,
            min_width: 0,
        }
    }

    // Set the alignment of the image element
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    // Set the minimum width of the element
    pub fn min_width(mut self, min_width: u16) -> Self {
        self.min_width = min_width;
        self
    }
}

// Trait for components
pub trait Component {
    fn background(&mut self) -> Option<Background>;
    fn position(&mut self) -> ComponentPosition;
    fn text(&mut self) -> Option<Text>;
    fn timeout(&mut self) -> Duration;
    fn event(&mut self); // TODO: Create event type
}
