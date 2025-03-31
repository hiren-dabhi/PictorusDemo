use crate::protocols::delay_protocol::{create_delay_protocol, DelayProtocolType};
use crate::protocols::i2c_protocol::{create_i2c_protocol, I2cProtocolType};
use crate::utils::PictorusError;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use embedded_hal_compat::Reverse;
use hd44780_driver::bus::I2CBus;
use hd44780_driver::{Cursor, CursorBlink, Display as LcdDisplay, DisplayMode, HD44780};
use log::debug;
use protocols::DisplayProtocol;
use ssd1306::{mode::BufferedGraphicsMode, prelude::*, I2CDisplayInterface, Ssd1306};

pub enum DisplayType {
    OLED,
    LCD,
}

const ERR_TYPE: &str = "UdpProtocol";

fn create_error(message: String) -> PictorusError {
    PictorusError::new(ERR_TYPE.into(), message)
}

pub struct DisplayConnection {
    ssd1306_display: Option<Ssd1306Display>,
    hd44780_display: Option<Hd44780Display>,
    last_display_value: String,
    last_rendered_offset: i64,
}

impl DisplayConnection {
    pub fn new(address: f64, display_type: &str) -> Result<Self, PictorusError> {
        let address = address as u16;
        let display_type = match display_type {
            "OLED" => DisplayType::OLED,
            "LCD" => DisplayType::LCD,
            _ => {
                return Err(create_error(format!(
                    "Unknown DisplayBlock type '{}'!",
                    display_type
                )))
            }
        };

        let mut ssd1306_display = None;
        let mut hd44780_display = None;
        match display_type {
            DisplayType::OLED => ssd1306_display = Some(Ssd1306Display::new(address)?),
            DisplayType::LCD => hd44780_display = Some(Hd44780Display::new(address)?),
        }

        Ok(DisplayConnection {
            ssd1306_display,
            hd44780_display,
            last_display_value: String::new(),
            last_rendered_offset: 0,
        })
    }
}

impl DisplayProtocol for DisplayConnection {
    fn render(&mut self, value: &str, x_offset: f64) {
        let x_offset = x_offset as i64;
        // Re-rendering is expensive, don't do it if string is identical
        if value == self.last_display_value && x_offset == self.last_rendered_offset {
            return;
        }

        if let Some(display) = &mut self.ssd1306_display {
            display.render(value, x_offset);
        }

        if let Some(display) = &mut self.hd44780_display {
            display.render(value, x_offset);
        }

        self.last_rendered_offset = x_offset;
        self.last_display_value = value.to_string();
    }
}

struct Ssd1306Display {
    display: Ssd1306<
        I2CInterface<I2cProtocolType>,
        DisplaySize128x32,
        BufferedGraphicsMode<DisplaySize128x32>,
    >,
    text_style: MonoTextStyle<'static, BinaryColor>,
}

#[cfg(feature = "std")]
impl Ssd1306Display {
    pub fn new(address: u16) -> Result<Self, PictorusError> {
        debug!("Creating Ssd1306Display for address {}", address);
        let i2c = create_i2c_protocol()?;
        let interface = I2CDisplayInterface::new_custom_address(i2c, address as u8);

        let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().map_err(|_| {
            create_error(format!(
                "Failed to initialize display at address: {}",
                address
            ))
        })?;
        display.set_brightness(Brightness::BRIGHTEST).ok();
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build();

        Ok(Ssd1306Display {
            display,
            text_style,
        })
    }

    fn render(&mut self, value: &str, x_offset: i64) {
        debug!("Rendering value: {}", value);
        self.display.clear();
        let start_point = Point::new(x_offset as i32, 16);
        Text::with_baseline(value, start_point, self.text_style, Baseline::Middle)
            .draw(&mut self.display)
            .unwrap();

        self.display.flush().unwrap();
    }
}

struct Hd44780Display {
    display: HD44780<I2CBus<I2cProtocolType>>,
    delay: DelayProtocolType,
}

impl Hd44780Display {
    pub fn new(address: u16) -> Result<Self, PictorusError> {
        debug!("Creating Hd44780Display for address {}", address);
        let i2c = create_i2c_protocol()?;
        let mut delay = create_delay_protocol();

        let mut display = HD44780::new_i2c(i2c, address as u8, &mut delay)
            .map_err(|_| create_error(format!("Failed to bind display to address: {}", address)))?;
        display.reset(&mut delay).map_err(|_| {
            create_error(format!("Failed to reset display at address: {}", address))
        })?;
        display.clear(&mut delay).map_err(|_| {
            create_error(format!("Failed to clear display at address: {}", address))
        })?;
        display
            .set_display_mode(
                DisplayMode {
                    display: LcdDisplay::On,
                    cursor_visibility: Cursor::Invisible,
                    cursor_blink: CursorBlink::Off,
                },
                &mut delay,
            )
            .map_err(|_| {
                create_error(format!(
                    "Failed to set display mode at address: {}",
                    address
                ))
            })?;

        Ok(Hd44780Display { display, delay })
    }
    fn render(&mut self, value: &str, x_offset: i64) {
        debug!("Rendering value: {}", value);
        self.display.clear(&mut self.delay).unwrap();
        self.display
            .set_cursor_pos(x_offset as u8, &mut self.delay)
            .unwrap();
        self.display.write_str(value, &mut self.delay).unwrap();
    }
}

pub fn create_display_protocol(
    address: f64,
    display_type: &str,
) -> Result<DisplayConnection, PictorusError> {
    let connection = DisplayConnection::new(address, display_type)?;
    Ok(connection)
}
