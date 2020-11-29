#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

use panic_semihosting as _; 
/*
// dev profile: easier to debug panics; can put a breakpoint on `rust_begin_unwind`
#[cfg(debug_assertions)]
use panic_halt as _;
// release profile: minimize the binary size of the application
#[cfg(not(debug_assertions))]
use panic_abort as _;
*/

use embedded_hal::digital::v2::OutputPin;
use rtic::app;
use rtic::cyccnt::U32Ext;
use stm32f1xx_hal::gpio::{gpioc::PC13, gpioa::PA5, Output, PushPull, State};
use stm32f1xx_hal::prelude::*;
#[cfg(feature = "semihosting")]
use cortex_m_semihosting::hprintln;

const PERIOD: u32 = 10_000_000;

macro_rules! sprintln {
    () => {
        #[cfg(feature = "semihosting")]
        hprintln!().unwrap();
    };
    ($s:expr) => {
        #[cfg(feature = "semihosting")]
        hprintln!($s).unwrap();
    };
    ($s:expr, $($tt:tt)*) => {
        #[cfg(feature = "semihosting")]
        hprintln!($s, $($tt)*).unwrap();
  };
}

// We need to pass monotonic = rtic::cyccnt::CYCCNT to use schedule feature fo RTIC
#[app(device = stm32f1xx_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    // Global resources (global variables) are defined here and initialized with the 
    // `LateResources` struct in init
    struct Resources {
        led: PC13<Output<PushPull>>,
        led_n: PA5<Output<PushPull>>,
        led_state: bool,
    }

    #[init(schedule = [blinker])]
    fn init(cx: init::Context) -> init::LateResources {

        // Enable cycle counter
        let mut core = cx.core;
        core.DWT.enable_cycle_counter();

        let device: stm32f1xx_hal::stm32::Peripherals = cx.device;

        // Setup clocks
        let mut flash = device.FLASH.constrain();
        let mut rcc = device.RCC.constrain();
        let mut _afio = device.AFIO.constrain(&mut rcc.apb2);
        let _clocks = rcc
            .cfgr
            .use_hse(8.mhz())
            .sysclk(72.mhz())
            .pclk1(36.mhz())
            .freeze(&mut flash.acr);

        // Setup LED on bluepill
        let mut gpioc = device.GPIOC.split(&mut rcc.apb2);
        let mut led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, State::Low);
        led.set_low().unwrap();
        // Setup LED on nucleo
        let mut gpioa = device.GPIOA.split(&mut rcc.apb2);
        let mut led_n = gpioa
            .pa5
            .into_push_pull_output_with_state(&mut gpioa.crl, State::Low);
        led_n.set_low().unwrap();

        // Schedule the blinking task
        cx.schedule.blinker(cx.start + PERIOD.cycles()).unwrap();

        init::LateResources { led: led, led_n: led_n, led_state: false}
    }

    #[task(resources = [led, led_n, led_state], schedule = [blinker])]
    fn blinker(cx: blinker::Context) {
        if *cx.resources.led_state {
            cx.resources.led.set_high().unwrap();
            cx.resources.led_n.set_high().unwrap();
            sprintln!("Led ON");
        } else {
            cx.resources.led.set_low().unwrap();
            cx.resources.led_n.set_low().unwrap();
            sprintln!("Led OFF");
        }
        *cx.resources.led_state ^= true;
        cx.schedule.blinker(cx.scheduled + PERIOD.cycles()).unwrap();
    }

    extern "C" {
        fn EXTI0();
    }
};
