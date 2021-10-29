#![no_std]
#![no_main]

extern crate panic_itm;

use cortex_m::peripheral;
use cortex_m::{asm, singleton};
use cortex_m_rt::entry;

use stm32f3_discovery::leds::Leds;
use stm32f3_discovery::stm32f3xx_hal::delay::Delay;
use stm32f3_discovery::stm32f3xx_hal::pac;
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
use stm32f3_discovery::stm32f3xx_hal::serial::Serial;
use stm32f3_discovery::switch_hal::{OutputSwitch, ToggleableOutputSwitch};

#[entry]
fn main() -> ! {
    let device_peripherals = pac::Peripherals::take().unwrap();

    // This is a workaround, so that the debugger will not disconnect
    // imidiatly on asm::wfi();
    // https://github.com/probe-rs/probe-rs/issues/350#issuecomment-740550519
    device_peripherals.DBGMCU.cr.modify(|_, w| {
        w.dbg_sleep().set_bit();
        w.dbg_standby().set_bit();
        w.dbg_stop().set_bit()
    });
    device_peripherals
        .RCC
        .ahbenr
        .modify(|_, w| w.dma1en().enabled());

    let mut reset_and_clock_control = device_peripherals.RCC.constrain();

    let core_peripherals = cortex_m::Peripherals::take().unwrap();
    let mut flash = device_peripherals.FLASH.constrain();
    let clocks = reset_and_clock_control.cfgr.freeze(&mut flash.acr);

    let mut delay = Delay::new(core_peripherals.SYST, clocks);

    // initialize user leds
    let mut gpioc = device_peripherals
        .GPIOC
        .split(&mut reset_and_clock_control.ahb);
    let mut gpioe = device_peripherals
        .GPIOE
        .split(&mut reset_and_clock_control.ahb);

    let pins_usart1 = (
        gpioc
            .pc4
            .into_af7_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl),
        gpioc
            .pc5
            .into_af7_push_pull(&mut gpioc.moder, &mut gpioc.otyper, &mut gpioc.afrl),
    );

    let serial = Serial::new(
        device_peripherals.USART1,
        pins_usart1,
        115200.Bd(),
        clocks,
        &mut reset_and_clock_control.apb2,
    );
    let (tx, _rx) = serial.split();

    let dma1 = device_peripherals
        .DMA1
        .split(&mut reset_and_clock_control.ahb);

    let tx_buf = singleton!(: [u8; 16] = *b"Hello bootloader").unwrap();
    let (tx_channel, _rx_channel) = (dma1.ch4, dma1.ch5);

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

    let mut sending = tx.write_all(tx_buf, tx_channel);
    loop {
        spin_leds(&mut leds, &mut delay);

        let (tx_buf, tx_channel, tx) = sending.wait();

        spin_leds(&mut leds, &mut delay);

        sending = tx.write_all(tx_buf, tx_channel);
    }

    boot_user(core_peripherals.SCB);
}

fn spin_leds(leds: &mut Leds, delay: &mut Delay) {
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

    for led in &mut sequence {
        led.toggle().unwrap();
        delay.delay_ms(100u16);
    }
}

#[inline]
pub fn user_vector_table() -> *mut u32 {
    extern "C" {
        static mut __suser: u32;
    }

    unsafe { &mut __suser }
}

fn boot_user(scb: peripheral::SCB) -> ! {
    let user_vector_table = user_vector_table();
    unsafe {
        scb.vtor.write(*user_vector_table);
        asm::bootload(user_vector_table);
    }
}
