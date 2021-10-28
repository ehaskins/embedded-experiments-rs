#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m_rt::entry;

use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::pac;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::switch_hal::{OutputSwitch, ToggleableOutputSwitch};

#[inline]
pub fn user_vector_table() -> *mut u32 {
    extern "C" {
        static mut __suser: u32;
    }

    unsafe { &mut __suser }
}

#[entry]
fn main() -> ! {
    let device_peripherals = pac::Peripherals::take().unwrap();
    let mut reset_and_clock_control = device_peripherals.RCC.constrain();

    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    // initialize user leds
    let mut gpioe = device_peripherals.GPIOE.split(&mut reset_and_clock_control.ahb);
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    let mut sequence = [
        &mut leds.ld3,
        &mut leds.ld5,
        &mut leds.ld7,
        &mut leds.ld9,
        &mut leds.ld10,
        &mut leds.ld8,
        &mut leds.ld6,
        &mut leds.ld4,
    ];

    loop {
        for led in &mut sequence {
            led.toggle().unwrap();
            delay.delay_ms(100u16);
        }
    }
}
