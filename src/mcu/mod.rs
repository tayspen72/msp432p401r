//==============================================================================
// Notes
//==============================================================================
// mcu::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use cortex_m;
use msp432p401r_pac;

use crate::config;

mod eusci;
pub mod gpio;
pub mod systick;
pub mod wdt;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
pub enum McuState {
	Idle
}

#[allow(dead_code)]
pub struct SystemClock{
	pub a_clk: u32,
	pub m_clk: u32,
	pub hsm_clk: u32,
	pub sm_clk: u32,
	pub b_clk: u32
}

//==============================================================================
// Variables
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Port{
	Port1,
	Port2,
	Port3,
	Port4,
	Port5,
	Port6,
	Port7,
	Port8,
	Port9,
	Port10,
	PortJ
}

const HFXT_CLK_IN: gpio::PinConfig = gpio::PinConfig {
	port: config::HFXCLK_IN_PORT,
	pin: config::HFXCLK_IN_PIN,
	direction: gpio::PinDirection::Input,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const HFXT_CLK_OUT: gpio::PinConfig = gpio::PinConfig {
	port: config::HFXCLK_OUT_PORT,
	pin: config::HFXCLK_OUT_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinHigh,
};
const LFXT_CLK_IN: gpio::PinConfig = gpio::PinConfig {
	port: config::LFXCLK_IN_PORT,
	pin: config::LFXCLK_IN_PIN,
	direction: gpio::PinDirection::Input,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinLow,
};
const LFXT_CLK_OUT: gpio::PinConfig = gpio::PinConfig {
	port: config::LFXCLK_OUT_PORT,
	pin: config::LFXCLK_OUT_PIN,
	direction: gpio::PinDirection::Output,
	pull: gpio::PinPull::PullDisabled,
	state: gpio::PinState::PinHigh,
};

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	let peripherals = msp432p401r_pac::Peripherals::take().unwrap();
	let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

	wdt::init(peripherals.WDT_A);

	// Enable all banks of SRAM and wait for SRAM_RDY to be set
	peripherals.SYSCTL.sys_sram_banken.write(|w| w.bnk7_en().set_bit());
	while peripherals.SYSCTL.sys_sram_banken.read().sram_rdy().is_sram_rdy_0() {};
	
	eusci::init(
		peripherals.EUSCI_A0,
		peripherals.EUSCI_A1,
		peripherals.EUSCI_A2,
		peripherals.EUSCI_A3,
		peripherals.EUSCI_B0,
		peripherals.EUSCI_B1,
		peripherals.EUSCI_B2,
		peripherals.EUSCI_B3
	);
	
	// adc::init(peripherals.SAADC);
	gpio::init(peripherals.DIO);
	// input::init(peripherals.GPIOTE);
	// i2c::init(peripherals.TWI1);
	// rtc::init(peripherals.RTC0, &peripherals.CLOCK, wake_interval);
	// spi::init(peripherals.SPI0);
	// spim::init(peripherals.SPIM0);
	// timer::init(peripherals.TIMER0);
	
	// Clock uses GPIO - initialize after gpio is initialized
	init_clock(peripherals.CS);

	// Ensure SYSTICK is using the correct clock when running. Init after clock
	systick::init(cortex_peripherals.SYST);

}

#[allow(dead_code)]
pub fn get_busy() -> McuState {
	McuState::Idle
}

#[allow(dead_code)]
pub fn restart() {
	cortex_m::peripheral::SCB::sys_reset();
}

//==============================================================================
// Private Functions
//==============================================================================
fn init_clock(clock: msp432p401r_pac::CS) {
	gpio::pin_setup(&HFXT_CLK_IN);
	gpio::set_pin_function_select(&HFXT_CLK_IN, 0b01);
	gpio::pin_setup(&HFXT_CLK_OUT);
	gpio::set_pin_function_select(&HFXT_CLK_OUT, 0b01);
	gpio::pin_setup(&LFXT_CLK_IN);
	gpio::set_pin_function_select(&LFXT_CLK_IN, 0b01);
	gpio::pin_setup(&LFXT_CLK_OUT);
	gpio::set_pin_function_select(&LFXT_CLK_OUT, 0b01);
	
	clock.cskey.write(|w| unsafe { w.cskey().bits(0x695A) });

	// Configure clock speeds
	clock.csctl1.write(|w| w
		// MCLK: Master Clock 48MHz
		.selm().selm_5()
		.divm().divm_0()
		// HSMCLK: Sub-Master Clock 24MHz
		.sels().sels_5()
		.divhs().divhs_1()
		// SMCLK: Low-Speed Master Clock 6MHz
		.divs().divs_4()
		// ACLK: Aux Clock 32.768 kHz
		.sela().sela_0()
		.diva().diva_0()
		// BCLK: Backup Clock 32.768 kHz
		.selb().selb_0()
	);
	
	// Enable HFXT and LFXT as external crystals
	clock.csctl2.write(|w| w
		.hfxtbypass().clear_bit()
		.hfxt_en().clear_bit()
		.hfxtfreq().hfxtfreq_6()
		.hfxtdrive().set_bit()
		
		.lfxtbypass().clear_bit()
		.lfxt_en().clear_bit()
		.lfxtdrive().lfxtdrive_0()
	);
	
	let mut status = clock.csstat.read().bits();
	
	while status & 0x1F000044 != 0x1F000044 {
		status = clock.csstat.read().bits();
	}
	
	// Lock the clock registers when finished
	clock.cskey.write(|w| unsafe { w.cskey().bits(0xFFFF) });
}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}