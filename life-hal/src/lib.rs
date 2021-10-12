// Copyright 2021 Simeon Miteff

#![no_std]

use ssd1306::{prelude::*, Ssd1306};
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use life::*;

/// life_i2c implements a common version of the Game Of Life for all embedded-hal I2C-connected SSD1306 displays
pub fn life_i2c<DI: WriteOnlyDataCommand>(interface: DI) -> ! {
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    let mut col = Colony::new();

    seed_gliders(&mut col);

    loop {
        col = generation(col);
        display.clear();
        for (x, y) in col.iter() {
            Pixel(Point::new(*x as i32, *y as i32), BinaryColor::On)
                .draw(&mut display)
                .unwrap();
        }
        display.flush().unwrap();
    }
}