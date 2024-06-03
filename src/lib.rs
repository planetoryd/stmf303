#![no_std]
// #![no_main]

use panic_probe as _;

use hal::{
    adc::{self, VoltageInternalReference},
    dac::Dac,
};

// use panic_rtt_target as _;
// use rtt_target::{
//     debug_rprintln, debug_rtt_init_default, debug_rtt_init_print, rprintln, rtt_init,
//     rtt_init_print, set_print_channel,
// };
// use rtt_target::{rtt_init_default, ChannelMode::BlockIfFull};
use cortex_m_semihosting::debug;

use stm32f3xx_hal::{self as hal, pac, prelude::*};
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

use rtic::app;
use rtic_monotonics::systick::prelude::*;

use systick_monotonic::{fugit::Duration, Systick};


#[app(device = stm32f3xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use hal::gpio::{Analog, Output, PushPull, PA4, PB10};
    use rtt_target::{rprint, rtt_init_default, rtt_init_print};
    systick_monotonic!(Mono, 1000);

    use rtt_target::ChannelMode::BlockIfFull;

    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: PB10<Output<PushPull>>,
        aout: PA4<Analog>,
        state: bool,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut flash = cx.device.FLASH.constrain();
        let mut rcc = cx.device.RCC.constrain();

        rtt_init_print!(BlockIfFull);

        let _clocks = rcc.cfgr.sysclk(48.MHz()).freeze(&mut flash.acr);

        let mut gpiob = cx.device.GPIOB.split(&mut rcc.ahb);
        let mut led = gpiob
            .pb10
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
        led.set_low().unwrap();

        Mono::start(cx.core.SYST, 48_000_000);

        let mut gpioa = cx.device.GPIOA.split(&mut rcc.ahb);
        let dac_pin = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);

        blink::spawn().unwrap();

        (
            Shared {},
            Local {
                led,
                state: false,
                aout: dac_pin,
            },
        )
    }

    #[task(local = [led, state])]
    async fn blink(cx: blink::Context) {
        loop {
            rprint!("blink");
            if *cx.local.state {
                cx.local.led.set_high().unwrap();
                *cx.local.state = false;
            } else {
                cx.local.led.set_low().unwrap();
                *cx.local.state = true;
            }
            Mono::delay(2000.millis()).await;
        }
    }
}

// #[entry]
// fn main() -> ! {
//     rtt_init_print!(BlockIfFull);

//     let dp = pac::Peripherals::take().unwrap();
//     let mut rcc = dp.RCC.constrain();
//     let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

//     rprintln!("init");

//     // On-board LED at PB10
//     let mut led = gpiob
//         .pb10
//         .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

//     let mut gpioa = dp.GPIOA.split(&mut rcc.ahb);
//     let mut dac_pin = gpioa.pa4.into_analog(&mut gpioa.moder, &mut gpioa.pupdr);
//     let mut dac1 = Dac::new(dp.DAC1, &mut rcc.apb1);
//     dac1.write_data(2000);
//     let clocks = rcc.cfgr.freeze(&mut dp.FLASH.constrain().acr);
//     let adc_co = adc::CommonAdc::new(dp.ADC1_2, &clocks, &mut rcc.ahb);
//     let tuple = (dp.ADC1, dp.ADC2);
//     let mut adc = adc::Adc::new(tuple.1, adc::config::Config::default(), &clocks, &adc_co);

//     loop {
//         led.toggle().unwrap();
//         let vv: u16 = adc.read(&mut dac_pin).unwrap();
//         rprintln!("DAC-V: {}", vv);
//         // dac1.write_data(vv);
//         asm::delay(2_000_000);
//     }
// }
