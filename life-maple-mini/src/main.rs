// Copyright 2021 James Waples
// Copyright 2021 Simeon Miteff

#![no_std]
#![no_main]

/// This code is adapted from the example in the ssd1306 repo, which was
/// written to run on the "blue pill":
///
/// https://github.com/jamwaffles/ssd1306/blob/HEAD/examples/image_i2c.rs
///
/// The maple mini has a slightly different pin-out. To try this yourself, wire:
/// - ssd1306 SDA to mini pin 15
/// - ssd1306 SCL to mini pin 16
/// - ssd1306 VCC to mini VCC
/// - ssd1306 GND to mini GND
/// - mini BUT to mini VCC
/// - mini pin 2 (boot) to mini GND
/// - mini GND to UART GND
/// - mini VIN to UART 3.3V
/// - mini pin 25 (rx1) to UART TXD
/// - mini pin 24 (tx1) to UART RXD
///
/// Follow the instructions in README.md for flashing the binary.

use cortex_m_rt::{entry, exception, ExceptionFrame};
use panic_halt as _;
use ssd1306::I2CDisplayInterface;
use stm32f1xx_hal::{
    i2c::{BlockingI2c, DutyCycle, Mode},
    prelude::*,
    stm32,
};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000.hz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    // Everything above here ^ is why I'm grateful for operating systems!

    let interface = I2CDisplayInterface::new(i2c);
    life_hal::life_i2c(interface);
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
