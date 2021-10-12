// Copyright 2021 Simeon Miteff

use linux_embedded_hal::I2cdev;
use ssd1306::I2CDisplayInterface;

fn main() {
    // Yes, it is this simple!
    //
    // See ../life-maple-mini/src/main.rs for comparison.

    let i2c = I2cdev::new("/dev/i2c-0").unwrap(); // Adjust path as needed
    let interface = I2CDisplayInterface::new(i2c);
    life_hal::life_i2c(interface);
}
