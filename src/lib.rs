#![no_std]
#![no_main]

use cortex_m::asm;
use cortex_m_rt::entry;
use hal::{
    adc::{self, VoltageInternalReference},
    dac::Dac,
};
use panic_halt as _;
use rtt_target::{
    debug_rprintln, debug_rtt_init_default, debug_rtt_init_print, rprintln, rtt_init,
    rtt_init_print, set_print_channel,
};
use rtt_target::{rtt_init_default, ChannelMode::BlockIfFull};
use stm32f3xx_hal::{self as hal, pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!(BlockIfFull);

    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    rprintln!("init");

    // On-board LED at PB10
    let mut led = gpiob
        .pb10
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
    let mut dac_pin = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
    let mut dac1 = Dac::new(dp.DAC1, &mut rcc.apb1);
    dac1.write_data(2000);
    let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
    let adc_co = adc::CommonAdc::new(dp.ADC1_2, &clocks, &mut rcc.ahb);
    let tuple = (dp.ADC1, dp.ADC2);
    let mut adc = adc::Adc::new(tuple.1, adc::config::Config::default(), &clocks, &adc_co);

    loop {
        led.toggle().unwrap();
        let vv: u16 = adc.read(&mut dac_pin).unwrap();
        rprintln!("DAC-V: {}", vv);
        // dac1.write_data(vv);
        asm::delay(2_000_000);
    }
}
