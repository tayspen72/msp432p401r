//==============================================================================
// Notes
//==============================================================================
// mcu::mod.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use cortex_m;
use msp432p401r;

pub mod gpio;
pub mod systick;

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

//==============================================================================
// Public Functions
//==============================================================================
pub fn init() {
	let peripherals = msp432p401r::Peripherals::take().unwrap();
	let cortex_peripherals = cortex_m::Peripherals::take().unwrap();

	systick::init(cortex_peripherals.SYST);

	// Enable all banks of SRAM and wait for SRAM_RDY to be set
	peripherals.SYSCTL.sys_sram_banken.write(|w| w.bnk7_en().set_bit());
	while peripherals.SYSCTL.sys_sram_banken.read().sram_rdy().is_sram_rdy_0() {};

	init_clock(peripherals.CS);
	
	// adc::init(peripherals.SAADC);
	gpio::init(peripherals.DIO);
	// input::init(peripherals.GPIOTE);
	// i2c::init(peripherals.TWI1);
	// rtc::init(peripherals.RTC0, &peripherals.CLOCK, wake_interval);
	// spi::init(peripherals.SPI0);
	// spim::init(peripherals.SPIM0);
	// timer::init(peripherals.TIMER0);
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
fn init_clock(clock: msp432p401r::CS) {
	clock.cskey.write(|w| unsafe { w.cskey().bits(0x695A) });


}

//==============================================================================
// Task Handler
//==============================================================================
pub fn task_handler() {

}