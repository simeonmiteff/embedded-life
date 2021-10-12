// Copyright 2021 Simeon Miteff

use life::*;
use std::time::{Duration, Instant};
use std::thread;
use embedded_graphics::{geometry::Point, pixelcolor::BinaryColor, prelude::*, Pixel};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

const TARGET_FPS: u32 = 20;

fn main() {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128_u32, 64_u32));
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    let mut window = Window::new("Conway's Game Of Life", &output_settings);

    let mut col = Colony::new();

    seed_gliders(&mut col);

    let frame_render_time = Duration::from_secs(1) / TARGET_FPS;

    'outer: loop {
        let start = Instant::now();
        col = generation(col);
        display.clear(BinaryColor::Off).unwrap();
        for cell in &col {
            Pixel(Point::new(cell.0 as i32, cell.1 as i32), BinaryColor::On)
                .draw(&mut display)
                .unwrap();
        }
        window.update(&display);
        let duration = start.elapsed();

        if duration < frame_render_time {
            let sleep_time = frame_render_time - duration;
            thread::sleep(sleep_time);
        }

        // Must service window events, otherwise the SDL window freezes
        for event in window.events() {
            if event == SimulatorEvent::Quit {
                break 'outer;
            }
        }
    }
}
