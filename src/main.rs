#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc, gpio::IO, Delay, gpio::Output, gpio::Input, soc::gpio::AnyPin};
use hal::{gpio::PullUp, gpio::PushPull};

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

    /* =========== PASSWORD ============ */
    const CORRECT_PASSWORD: &str = "2734A5";
    let mut password: &str = "";

    let mut delay = Delay::new(&clocks);
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    /* =========== ROWS ============ */
    let mut row_gpio = [
        io.pins.gpio4.into_pull_up_input().degrade(),
        io.pins.gpio5.into_pull_up_input().degrade(),
        io.pins.gpio6.into_pull_up_input().degrade(),
        io.pins.gpio7.into_pull_up_input().degrade(),
    ];

    /* =========== COLUMNS ============ */
    let mut column_gpio = [
        io.pins.gpio15.into_push_pull_output().degrade(),
        io.pins.gpio16.into_push_pull_output().degrade(),
        io.pins.gpio17.into_push_pull_output().degrade(),
        io.pins.gpio18.into_push_pull_output().degrade(),
    ];

    // Set columns to low
    column_gpio.iter_mut().for_each(|column| {
        column.set_low().unwrap();
    });

    loop {
        println!("{:?}", get_key_pressed(&mut column_gpio, &mut row_gpio));
    }
}

fn get_key_pressed(column_gpio: &mut [AnyPin<Output<PushPull>>; 4], row_gpio: &mut [AnyPin<Input<PullUp>>; 4]) -> Option<char> {

    /* =========== KEYBOARD =========== */
    const KEYBOARD: [[char; 4]; 4] = [['1', '2', '3', 'A'],
                                      ['4', '5', '6', 'B'],
                                      ['7', '8', '9', 'C'],
                                      ['*', '0', '#', 'D']];

    let mut actual_row = 0;
    let mut actual_column = 0;

    for column in column_gpio.iter_mut() {
        column.set_low().unwrap();

        for row in row_gpio.iter_mut() {
            if row.is_low().unwrap() {
                column.set_high().unwrap();
                return Some(KEYBOARD[actual_row][actual_column]);
            }

            actual_row += 1;
        }

        actual_row = 0;
        actual_column += 1;
        column.set_high().unwrap();
    }

    None
}
