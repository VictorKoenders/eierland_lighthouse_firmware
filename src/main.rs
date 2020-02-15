#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use stm32f1xx_hal::{
    delay::Delay,
    device::{CorePeripherals, Peripherals},
    prelude::*,
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.constrain();

    let mut flash = peripherals.FLASH.constrain();
    let core_peripherals = CorePeripherals::take().unwrap();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    let mut gpioa = peripherals.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = peripherals.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_open_drain_output(&mut gpioc.crh);

    // These pins are debugger pins and cannot be switched to push_pull_output
    // let mut r0 = gpiob.pb3.into_push_pull_output(&mut gpiob.crl);
    // let mut g0 = gpiob.pb4.into_push_pull_output(&mut gpiob.crl);
    let _b0 = gpiob.pb5.into_open_drain_output(&mut gpiob.crl);

    let mut ledw0 = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
    let mut ledw1 = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);

    let led0 = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
    let led1 = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
    let led2 = gpiob.pb10.into_push_pull_output(&mut gpiob.crh);
    let led3 = gpiob.pb11.into_push_pull_output(&mut gpiob.crh);
    let led4 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let led5 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let led6 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let led7 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

    let _r1 = gpioa.pa8.into_open_drain_output(&mut gpioa.crh);
    let _g1 = gpioa.pa9.into_open_drain_output(&mut gpioa.crh);
    let _b1 = gpioa.pa10.into_open_drain_output(&mut gpioa.crh);

    let but0 = gpiob.pb0.into_pull_up_input(&mut gpiob.crl);
    let but1 = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);

    let mut leds = [
        led0.downgrade(),
        led1.downgrade(),
        led2.downgrade(),
        led3.downgrade(),
        led7.downgrade(),
        led6.downgrade(),
        led5.downgrade(),
        led4.downgrade(),
    ];

    led.set_low().unwrap();

    const RANGE: usize = 3;
    for led in leds.iter_mut().take(RANGE) {
        led.set_high().unwrap();
    }
    ledw0.set_low().unwrap();
    ledw1.set_low().unwrap();

    let mut tail = 0;
    let mut head = RANGE;

    let mut direction = Direction::Right;

    loop {
        if but0.is_low().unwrap_or(false) {
            direction = Direction::Right;
        }
        if but1.is_low().unwrap_or(false) {
            direction = Direction::Left;
        }
        match direction {
            Direction::Right => {
                leds[tail].set_low().unwrap();
                leds[head].set_high().unwrap();

                tail = (tail + 1) % leds.len();
                head = (head + 1) % leds.len();

                if head == 0 {
                    ledw0.set_high().unwrap();
                    ledw1.set_high().unwrap();
                }

                if head == 2 {
                    ledw0.set_low().unwrap();
                    ledw1.set_low().unwrap();
                }
            }
            Direction::Left => {
                leds[tail].set_high().unwrap();
                leds[head].set_low().unwrap();

                tail = if tail == 0 { leds.len() } else { tail } - 1;
                head = if head == 0 { leds.len() } else { head } - 1;

                if head == 2 {
                    ledw0.set_high().unwrap();
                    ledw1.set_high().unwrap();
                }

                if head == 0 {
                    ledw0.set_low().unwrap();
                    ledw1.set_low().unwrap();
                }
            }
        }

        delay.delay_ms(150_u32);
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}
