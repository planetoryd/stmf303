#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f3xx_hal::{self as hal, pac, prelude::*};

#[entry]
fn main() -> ! {
      let dp = pac::Peripherals::take().unwrap();

      let mut rcc = dp.RCC.constrain();
      let mut gpioe = dp.GPIOA.split(&mut rcc.ahb);

      let mut led = gpioe
            .pa1
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

      loop {
            led.toggle().unwrap();
            asm::delay(8_000_000);
      }
}