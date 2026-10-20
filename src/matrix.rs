//! Scanner for the HHKB Pro 2 Topre daughter board.
//!
//! Each scan selects the 64 cells through the six BIT lines, drives HYS with the
//! cell's current state (Topre hysteresis), pulses STROBE and reads KEY. Topre
//! cannot raise an interrupt, so this always polls, slower the longer it idles.

use cortex_m::peripheral::{DCB, DWT};
use embassy_nrf::Peri;
use embassy_nrf::gpio::{AnyPin, Flex, Level, Output, OutputDrive, Pull};
use embassy_time::{Duration, Instant, Timer};
use rmk::event::KeyboardEvent;
use rmk::macros::input_device;

use crate::layout::MATRIX_TO_LAYOUT;

const CYCLES_PER_US: u32 = 64;

// 2 ms is the measured minimum.
const WARM_UP: Duration = Duration::from_millis(3);
const RELAX_CYCLES: u32 = 6 * CYCLES_PER_US;
const SETTLE_CYCLES: u32 = 5 * CYCLES_PER_US;
const STROBE_PULSE_CYCLES: u32 = CYCLES_PER_US / 2;

const ACTIVE_INTERVAL: Duration = Duration::from_millis(3);
// Idle and sleep scans can miss a tap shorter than one scan period.
const IDLE_INTERVAL: Duration = Duration::from_millis(21);
const SLEEP_INTERVAL: Duration = Duration::from_millis(250);
const IDLE_AFTER: Duration = Duration::from_secs(30);
const SLEEP_AFTER: Duration = Duration::from_secs(30 + 600);

/// The daughter board header.
pub struct Pins {
    pub power: Peri<'static, AnyPin>,
    pub key: Peri<'static, AnyPin>,
    pub hys: Peri<'static, AnyPin>,
    pub strobe: Peri<'static, AnyPin>,
    /// BIT0 to BIT2 select the row, BIT3 to BIT5 the column.
    pub bits: [Peri<'static, AnyPin>; 6],
}

#[input_device(publish = KeyboardEvent)]
pub struct HhkbMatrix {
    power: Output<'static>,
    key: Flex<'static>,
    hys: Output<'static>,
    strobe: Output<'static>,
    bits: [Output<'static>; 6],
    dcb: DCB,
    dwt: DWT,
    /// Confirmed key state, one bit per cell, numbered row * 8 + column.
    pressed: u64,
    /// Cells whose last scan disagreed with `pressed`.
    pending: u64,
    /// Confirmed changes not yet reported.
    unreported: u64,
    last_activity: Instant,
}

impl HhkbMatrix {
    pub fn new(pins: Pins, dcb: DCB, dwt: DWT) -> Self {
        let mut key = Flex::new(pins.key);
        key.set_as_disconnected();
        Self {
            // POWER feeds the board straight from the GPIO, so it needs high drive.
            power: Output::new(pins.power, Level::Low, OutputDrive::Standard0HighDrive1),
            key,
            hys: low_output(pins.hys),
            strobe: low_output(pins.strobe),
            bits: pins.bits.map(low_output),
            dcb,
            dwt,
            pressed: 0,
            pending: 0,
            unreported: 0,
            last_activity: Instant::now(),
        }
    }

    async fn read_keyboard_event(&mut self) -> KeyboardEvent {
        loop {
            while self.unreported != 0 {
                let cell = self.unreported.trailing_zeros() as usize;
                self.unreported &= !(1 << cell);
                if let Some((row, col)) = MATRIX_TO_LAYOUT[cell / 8][cell % 8] {
                    return KeyboardEvent::key(row, col, self.pressed & (1 << cell) != 0);
                }
            }
            if self.pending == 0 {
                Timer::after(self.poll_interval()).await;
            }
            self.scan().await;
        }
    }

    fn poll_interval(&self) -> Duration {
        let idle_for = self.last_activity.elapsed();
        if idle_for >= SLEEP_AFTER {
            SLEEP_INTERVAL
        } else if idle_for >= IDLE_AFTER {
            IDLE_INTERVAL
        } else {
            ACTIVE_INTERVAL
        }
    }

    async fn scan(&mut self) {
        if self.power.is_set_low() {
            self.power_on().await;
        }
        let read = self.sample();

        // Only accept a change once two scans in a row agree. Without this,
        // bluetooth interrupts make ~3% of keys type twice.
        let changed = read ^ self.pressed;
        let confirmed = changed & self.pending;
        self.pending = changed & !confirmed;
        self.pressed ^= confirmed;
        self.unreported |= confirmed;
        if confirmed != 0 {
            self.last_activity = Instant::now();
        }

        // A power-up costs as much as ~10 ms of staying on, so stay on between
        // active scans and for the rescan that confirms a change.
        if self.pending == 0 && self.poll_interval() != ACTIVE_INTERVAL {
            self.power_off();
        }
    }

    async fn power_on(&mut self) {
        self.key.set_as_input(Pull::Up);
        self.strobe.set_high();
        self.power.set_high();
        Timer::after(WARM_UP).await;
    }

    /// Reads every cell. The board must be powered.
    fn sample(&mut self) -> u64 {
        // The cycle counter that times the waits only runs while trace is enabled.
        self.dcb.enable_trace();
        self.dwt.enable_cycle_counter();
        let mut read = 0;
        for row in 0..8 {
            for col in 0..8 {
                let cell = row * 8 + col;
                let select = row | (col << 3);
                for (i, bit) in self.bits.iter_mut().enumerate() {
                    bit.set_level(Level::from(select & (1 << i) != 0));
                }
                self.hys
                    .set_level(Level::from(self.pressed & (1 << cell) != 0));
                delay_cycles(RELAX_CYCLES);
                let down = critical_section::with(|_| {
                    self.strobe.set_low();
                    delay_cycles(STROBE_PULSE_CYCLES);
                    self.strobe.set_high();
                    delay_cycles(SETTLE_CYCLES);
                    self.key.is_low()
                });
                self.hys.set_low();
                read |= u64::from(down) << cell;
            }
        }
        self.dwt.disable_cycle_counter();
        self.dcb.disable_trace();
        read
    }

    /// All pins go low with the power, so none leaks current into the board.
    fn power_off(&mut self) {
        for bit in &mut self.bits {
            bit.set_low();
        }
        self.key.set_as_disconnected();
        self.power.set_low();
        self.strobe.set_low();
    }
}

fn low_output(pin: Peri<'static, AnyPin>) -> Output<'static> {
    Output::new(pin, Level::Low, OutputDrive::Standard)
}

/// Busy-waits on the DWT cycle counter. KEY is valid for only ~20 us after
/// STROBE, too tight for counted loops, whose speed depends on the flash cache.
fn delay_cycles(cycles: u32) {
    let start = DWT::cycle_count();
    while DWT::cycle_count().wrapping_sub(start) < cycles {}
}
