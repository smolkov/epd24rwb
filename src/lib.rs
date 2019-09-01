#![feature(async_await)]

pub mod config;

use async_std::prelude::*;
use async_std::io;
use async_std::fs;
use async_std::task;
use std::time::Duration;

// use config::Config;
// use super::rpinky;
/// Linux 
use linux_embedded_hal::spidev::{self, SpidevOptions};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::Delay;
use linux_embedded_hal::{Pin, Spidev};

use ssd1675::{Builder, Color, Dimensions, Display, GraphicDisplay, Rotation};
// use ssd1675::graphics::*;
// use ssd

// Graphics
// use embedded_graphics::*;
use embedded_graphics::coord::Coord;
use embedded_graphics::prelude::*;
use embedded_graphics::Drawing;

// Font
extern crate profont;
use profont::{ProFont12Point, ProFont14Point, ProFont24Point, ProFont9Point};

use std::process::Command;
// use std::{fs, io};



// Activate SPI, GPIO in raspi-config needs to be run with sudo because of some sysfs_gpio
// permission problems and follow-up timing problems
// see https://github.com/rust-embedded/rust-sysfs-gpio/issues/5 and follow-up issues

const ROWS: u16 = 212;
const COLS: u8 = 104;

#[rustfmt::skip]
const LUT: [u8; 70] = [
    // Phase 0     Phase 1     Phase 2     Phase 3     Phase 4     Phase 5     Phase 6
    // A B C D     A B C D     A B C D     A B C D     A B C D     A B C D     A B C D
    0b01001000, 0b10100000, 0b00010000, 0b00010000, 0b00010011, 0b00000000, 0b00000000,  // LUT0 - Black
    0b01001000, 0b10100000, 0b10000000, 0b00000000, 0b00000011, 0b00000000, 0b00000000,  // LUTT1 - White
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // IGNORE
    0b01001000, 0b10100101, 0b00000000, 0b10111011, 0b00000000, 0b00000000, 0b00000000,  // LUT3 - Red
    0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,  // LUT4 - VCOM

    // Duration            |  Repeat
    // A   B     C     D   |
    64,   12,   32,   12,    6,   // 0 Flash
    16,   8,    4,    4,     6,   // 1 clear
    4,    8,    8,    16,    16,  // 2 bring in the black
    2,    2,    2,    64,    32,  // 3 time for red
    2,    2,    2,    2,     2,   // 4 final black sharpen phase
    0,    0,    0,    0,     0,   // 5
    0,    0,    0,    0,     0    // 6
];



pub async fn start() -> io::Result<()> {  
    // Configure SPI
    let mut spi = Spidev::open("/dev/spidev0.0").expect("SPI device");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(4_000_000)
        .mode(spidev::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("SPI configuration");

    // https://pinout.xyz/pinout/inky_phat
    // Configure Digital I/O Pins
    let cs = Pin::new(8); // BCM8
    cs.export().expect("cs export");
    while !cs.is_exported() {}
    cs.set_direction(Direction::Out).expect("CS Direction");
    cs.set_value(1).expect("CS Value set to 1");

    let busy = Pin::new(17); // BCM17
    busy.export().expect("busy export");
    while !busy.is_exported() {}
    busy.set_direction(Direction::In).expect("busy Direction");

    let dc = Pin::new(22); // BCM22
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    let reset = Pin::new(27); // BCM27
    reset.export().expect("reset export");
    while !reset.is_exported() {}
    reset
        .set_direction(Direction::Out)
        .expect("reset Direction");
    reset.set_value(1).expect("reset Value set to 1");
    println!("Pins configured");

    // Initialise display controller
    let mut delay = Delay {};

    let controller = ssd1675::Interface::new(spi, cs, busy, dc, reset);

    let mut black_buffer = [0u8; ROWS as usize * COLS as usize / 8];
    let mut red_buffer = [0u8; ROWS as usize * COLS as usize / 8];
    let config = Builder::new()
        .dimensions(Dimensions {
            rows: ROWS,
            cols: COLS,
        })
        .rotation(Rotation::Rotate270)
        .lut(&LUT)
        .build()
        .expect("invalid configuration");
    let display = Display::new(controller, config);
    let mut display = GraphicDisplay::new(display, &mut black_buffer, &mut red_buffer);

    // Main loop. Displays CPU temperature, uname, and uptime every minute with a red Raspberry Pi
    // header.
    loop {
        display.reset(&mut delay).expect("error resetting display");
        println!("Reset and initialised");
        let one_minute = Duration::from_secs(60);

        display.clear(Color::White);
        println!("Clear");

        display.draw(
            ProFont24Point::render_str("Irrigatron")
                .with_stroke(Some(Color::Red))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, -4))
                .into_iter(),
        );
    
        if let Ok(cpu_temp) = read_cpu_temp().await {
            display.draw(
                ProFont14Point::render_str("CPU Temp:")
                    .with_stroke(Some(Color::Black))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 30))
                    .into_iter(),
            );
            display.draw(
                ProFont12Point::render_str(&format!("{:.1}°C", cpu_temp))
                    .with_stroke(Some(Color::Black))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(95, 34))
                    .into_iter(),
            );
        }

        if let Some(uptime) = read_uptime() {
            display.draw(
                ProFont9Point::render_str(uptime.trim())
                    .with_stroke(Some(Color::Black))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 93))
                    .into_iter(),
            );
        }

        if let Some(uname) = read_uname() {
            display.draw(
                ProFont9Point::render_str(uname.trim())
                    .with_stroke(Some(Color::Black))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 84))
                    .into_iter(),
            );
        }

        display.update(&mut delay).expect("error updating display");
        println!("Update...");

        println!("Finished - going to sleep");
        display.deep_sleep()?;

        task::sleep(Duration::from_secs(1)).await;
        display.reset(&mut delay).expect("error resetting display");
        println!("Szene2");
        let one_minute = Duration::from_secs(60);

        display.clear(Color::White);
        println!("Agni pagni");

        display.draw(
            ProFont14Point::render_str("Liebe Agni-Pagni")
                .with_stroke(Some(Color::Red))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, -4))
                .into_iter(),
        );
        display.draw(
            ProFont14Point::render_str("zum Geburtstag")
                .with_stroke(Some(Color::Red))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 13))
                .into_iter(),
        );
        display.draw(
        ProFont14Point::render_str("wir wünschen dir")
                    .with_stroke(Some(Color::Red))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 27))
                    .into_iter(),
            );
        display.draw(
        ProFont12Point::render_str("alles Gute und viel Glück")
                    .with_stroke(Some(Color::Red))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 42))
                    .into_iter(),
            );
        display.draw(
                ProFont9Point::render_str("Irrigatron bittet dir seine Dinst")
                    .with_stroke(Some(Color::Black))
                    .with_fill(Some(Color::White))
                    .translate(Coord::new(1, 57))
                    .into_iter(),
            );
        display.draw(
            ProFont9Point::render_str("Geburehfreie und treu")
                .with_stroke(Some(Color::Black))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 66))
                .into_iter(),
        );
        display.draw(
            ProFont9Point::render_str("arbeitet auf deine auf Plantagen")
                .with_stroke(Some(Color::Black))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 76))
                .into_iter(),
        );
        display.draw(
            ProFont9Point::render_str("Unkompliziert und gute Zuhorer,")
                .with_stroke(Some(Color::Black))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 85))
                .into_iter(),
        );
        display.draw(
            ProFont9Point::render_str("Kostonlose Updates und Nachrustung")
                .with_stroke(Some(Color::Red))
                .with_fill(Some(Color::White))
                .translate(Coord::new(1, 94))
                .into_iter(),
        );
         
        display.update(&mut delay).expect("error updating display");
        println!("Update...");

        println!("Finished - going to sleep");
        display.deep_sleep()?;

        task::sleep(one_minute);
    }
}

async fn read_cpu_temp() -> io::Result<f64> {
    fs::read_to_string("/sys/class/thermal/thermal_zone0/temp").await?
        .trim()
        .parse::<i32>()
        .map(|temp| temp as f64 / 1000.)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))
}

fn read_uptime() -> Option<String> {
    Command::new("uptime")
        .arg("-p")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
}

fn read_uname() -> Option<String> {
    Command::new("uname")
        .arg("-smr")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok()
            } else {
                None
            }
        })
}