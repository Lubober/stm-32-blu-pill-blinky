#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f1xx_hal::{pac, prelude::*};

#[entry]
fn main() -> !{
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc);

    let mut led = gpioa.pa0.into_push_pull_output(&mut gpioa.crl);

    let mut delay = cp.SYST.delay(&rcc.clocks);

    loop {
        led.set_high();
        delay.delay_ms(500_u32);

        led.set_low();
        delay.delay_ms(500_u32);
    }
}
