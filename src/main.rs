#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, gpio::IO, Delay};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();
    println!("Hello world!");

    let mut password: &str = "";
    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    /* =========== ROWS ============ */
    let row_gpio = [
        io.pins.gpio4.into_pull_up_input().degrade(),
        io.pins.gpio5.into_pull_up_input().degrade(),
        io.pins.gpio6.into_pull_up_input().degrade(),
        io.pins.gpio7.into_pull_up_input().degrade(),
    ];

    /* =========== COLUMNS ============ */
    let column_gpio = [
        io.pins.gpio15.into_open_drain_output().degrade(),
        io.pins.gpio16.into_open_drain_output().degrade(),
        io.pins.gpio17.into_open_drain_output().degrade(),
        io.pins.gpio18.into_open_drain_output().degrade(),
    ];


    /* =========== KEYBOARD =========== */
    let keyboard = [["1", "2", "3", "A"],
                    ["4", "5", "6", "B"],
                    ["7", "8", "9", "C"],
                    ["*", "0", "#", "D"]];

    loop {
        /*for mut column in &column_gpio {
            column.set_high().unwrap();

            for mut row in &row_gpio {
                if row.is_low().unwrap() {
                    println!("Something pressed")
                }
            }

            column.set_low().unwrap();
        }*/

        column_gpio.iter().for_each(|column| {
            // column.set_high().unwrap();

            row_gpio.iter().for_each(|row| {
                if row.is_low().unwrap() {
                    println!("Key pressed");
                }
            });

            // column.set_low().unwrap();
        });
    }
}
