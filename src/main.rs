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
    let mut row_gpio = [
        io.pins.gpio4.into_pull_up_input(),
        io.pins.gpio5.into_pull_up_input(),
        io.pins.gpio6.into_pull_up_input(),
        io.pins.gpio7.into_pull_up_input(),
    ];

    /* =========== COLUMNS ============ */
    let mut column_gpio = [
        io.pins.gpio15.into_push_pull_output(),
        io.pins.gpio16.into_push_pull_output(),
        io.pins.gpio17.into_push_pull_output(),
        io.pins.gpio18.into_push_pull_output(),
    ];


    /* =========== KEYBOARD =========== */
    let keyboard = [["1", "2", "3", "A"],
                    ["4", "5", "6", "B"],
                    ["7", "8", "9", "C"],
                    ["*", "0", "#", "D"]];

    loop {
        let mut row: Option<u8> = None;
        let mut column: Option<u8> = None;

        for (column, column_gpio) in column_gpio {
            column_gpio.set_high().unwrap();

            for (row, row_gpio) in row_gpio {
                if row_gpio.is_low().unwrap() {
                    println!("{} : {}", row, column);
                }
            }

            column_gpio.set_high().unwrap();
        }
    }
}
