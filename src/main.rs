#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use stm32f1xx_hal::{device::Peripherals, prelude::*};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let mut rcc = peripherals.RCC.constrain();

    // let mut flash = peripherals.FLASH.constrain();
    // let core_peripherals = CorePeripherals::take().unwrap();
    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let mut delay = Delay::new(core_peripherals.SYST, clocks);

    let mut gpioa = peripherals.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = peripherals.GPIOB.split(&mut rcc.apb2);
    let mut gpioc = peripherals.GPIOC.split(&mut rcc.apb2);

    let led = gpioc.pc13.into_open_drain_output(&mut gpioc.crh);

    // These pins are debugger pins and cannot be switched to push_pull_output
    // let mut r0 = gpiob.pb3.into_push_pull_output(&mut gpiob.crl);
    // let mut g0 = gpiob.pb4.into_push_pull_output(&mut gpiob.crl);
    let b0 = gpiob.pb5.into_open_drain_output(&mut gpiob.crl);

    let ledw0 = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);
    let ledw1 = gpiob.pb7.into_push_pull_output(&mut gpiob.crl);

    let led0 = gpiob.pb8.into_push_pull_output(&mut gpiob.crh);
    let led1 = gpiob.pb9.into_push_pull_output(&mut gpiob.crh);
    let led2 = gpiob.pb10.into_push_pull_output(&mut gpiob.crh);
    let led3 = gpiob.pb11.into_push_pull_output(&mut gpiob.crh);
    let led4 = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let led5 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    let led6 = gpiob.pb14.into_push_pull_output(&mut gpiob.crh);
    let led7 = gpiob.pb15.into_push_pull_output(&mut gpiob.crh);

    let r1 = gpioa.pa8.into_open_drain_output(&mut gpioa.crh);
    let g1 = gpioa.pa9.into_open_drain_output(&mut gpioa.crh);
    let b1 = gpioa.pa10.into_open_drain_output(&mut gpioa.crh);

    let but0 = gpiob.pb0.into_pull_up_input(&mut gpiob.crl);
    let but1 = gpiob.pb1.into_pull_up_input(&mut gpiob.crl);

    let mut push_leds = [
        ledw0.downgrade(),
        ledw1.downgrade(),
        led0.downgrade(),
        led1.downgrade(),
        led2.downgrade(),
        led3.downgrade(),
        led4.downgrade(),
        led5.downgrade(),
        led6.downgrade(),
        led7.downgrade(),
    ];

    let mut drain_leds = [
        led.downgrade(),
        b0.downgrade(),
        r1.downgrade(),
        g1.downgrade(),
        b1.downgrade(),
    ];

    loop {
        for led in push_leds.iter_mut() {
            led.set_high().unwrap();
        }
        for led in drain_leds.iter_mut() {
            led.set_low().unwrap();
        }

        while !but0.is_low().unwrap_or(false) {
            asm::nop();
        }

        for led in push_leds.iter_mut() {
            led.set_low().unwrap();
        }
        for led in drain_leds.iter_mut() {
            led.set_high().unwrap();
        }
        while !but1.is_low().unwrap_or(false) {
            asm::nop();
        }
    }
}
