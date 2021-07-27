//==============================================================================
// Notes
//==============================================================================
// mcu::adc.rs
// ADC Driver

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::mcu;
use crate::mcu::gpio;
use msp432p401r_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Channel{
	A0 = 0,
	A1 = 1,
	A2 = 2,
	A3 = 3,
	A4 = 4,
	A5 = 5,
	A6 = 6,
	A7 = 7,
	A8 = 8,
	A9 = 9,
	A10 = 10,
	A11 = 11,
	A12 = 12,
	A13 = 13,
	A14 = 14,
	A15 = 15,
	A16 = 16,
	A17 = 17,
	A18 = 18,
	A19 = 19,
	A20 = 20,
	A21 = 21,
	Temperature = 22,
	Battery = 23
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
enum ChannelMap{
	ChMap0,
	ChMap1,
	ChMap2,
	ChMap3,
	TcMap,
	BatMap
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum TriggerSource{
	Software = 0,
	Ta0c1 = 1,
	Ta0c2 = 2,
	Ta1c1 = 3,
	Ta1c2 = 4,
	Ta2c1 = 5,
	Ta2c2 = 6,
	Ta3c1 = 7,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum Resolution{
	B8 = 0xFF,
	B10 = 0x3FF,
	B12 = 0xFFF,
	B14	= 0x3FFF
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub struct Adc{
	pub port: mcu::Port,
	pub pin: u8,
	pub channel: Channel,
	pub function_select: u8,
	pub resolution: Resolution,

}

//==============================================================================
// Variables
//==============================================================================
static ADC_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::ADC14>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

const TEMPERATURE_ADC: Adc = Adc {
	port: mcu::Port::PortDisabled,
	pin: 0,
	channel: Channel::Temperature,
	function_select: 0,
	resolution: Resolution::B14
};

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(adc: msp432p401r_pac::ADC14) {
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| ADC_HANDLE.borrow(cs).replace(Some(adc)));

	configure(&TEMPERATURE_ADC);

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn configure(adc: &Adc) {
	gpio::pin_setup(&gpio::PinConfig {
		port: adc.port,
		pin: adc.pin,
		direction: gpio::PinDirection::Input,
		pull: gpio::PinPull::PullDisabled,
		state: gpio::PinState::PinLow
	});
	gpio::set_pin_function_select(
		&gpio::PinConfig {
			port: adc.port,
			pin: adc.pin,
			direction: gpio::PinDirection::Input,
			pull: gpio::PinPull::PullDisabled,
			state: gpio::PinState::PinLow
		},
		adc.function_select
	);

	free(|cs| {
		if let Some(ref mut adc14) = ADC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Be sure to be disabled, especially for config
			adc14.adc14ctl0.write(|w| w.adc14enc().clear_bit());

			adc14.adc14ctl0.write(|w| w
				.adc14pdiv().adc14pdiv_0()
				// .adc14shs().bits(TriggerSource::Software as u8)
				.adc14div().adc14div_0()
				.adc14ssel().adc14ssel_4()
				// .adc14conseq().adc14conseq_0()
				.adc14sht1().adc14sht1_7()
				.adc14sht0().adc14sht0_7()
				.adc14on().adc14on_1()
				// .adc14enc().clear_bit()
			);

			// Enable interrupt flag for completion monitoring
			adc14.adc14ier0.modify(|r, w| unsafe { w.bits(r.bits() | (1 << adc.channel as u8)) });

			// Do not re-enable when finished
		}
	});
}

#[allow(dead_code)]
pub fn get_temperature() -> i8 {
	// Temperature graph seems to be appx:
	//	y = 2x + 685mV
	//	-> 
	//	temp(C) = { ADC(mV) - 685mV } / 2
	let read = read_ref(&TEMPERATURE_ADC, 3.3);

	((read - 685.0) / 2.0) as i8
}

#[allow(dead_code)]
pub fn read(adc: &Adc) -> u16 {
	free(|cs| {
		if let Some(ref mut adc14) = ADC_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			// Assign object config
			let channel_map = get_channel_map(adc.channel);
			adc14.adc14ctl1.write(|w| unsafe { w
				.adc14ch3map().bit(channel_map == ChannelMap::ChMap3)
				.adc14ch2map().bit(channel_map == ChannelMap::ChMap2)
				.adc14ch1map().bit(channel_map == ChannelMap::ChMap1)
				.adc14ch0map().bit(channel_map == ChannelMap::ChMap0)
				.adc14tcmap().bit(channel_map == ChannelMap::TcMap)
				.adc14batmap().bit(channel_map == ChannelMap::BatMap)
				.adc14cstartadd().bits(adc.channel as u8)
				.adc14res().bits(adc.resolution as u8)
				.adc14df().clear_bit()
				.adc14refburst().set_bit()
				.adc14pwrmd().adc14pwrmd_0()
			} );

			adc14.adc14mctl[adc.channel as usize].write(|w| w
				.adc14dif().clear_bit()
				.adc14vrsel().adc14vrsel_0()
				.adc14eos().set_bit()
				.adc14inch().bits(adc.channel as u8)
			);

			// Clear the conversion flag before starting
			adc14.adc14clrifgr0.write(|w| unsafe { w.bits(1<< adc.channel as u8) });

			// Set software trigger to start read
			adc14.adc14ctl0.modify(|_, w| w
				.adc14sc().set_bit()
			);
			
			// Wait for config to finish
			while adc14.adc14ifgr0.read().bits() & (1 << adc.channel as u8) == 0 {}
			// while adc14.adc14ctl0.read().adc14busy().bit() {}

			adc14.adc14mem[adc.channel as usize].read().conversion_results().bits()
		}
		else {
			0
		}
	})
}

#[allow(dead_code)]
pub fn read_ref(adc: &Adc, v_ref: f32) -> f32 {
	let read = read(adc);
	(read as f32) * v_ref / (adc.resolution as u16 as f32) 
}

//==============================================================================
// Private Functions
//==============================================================================
fn get_channel_map(channel: Channel) -> ChannelMap {
	match channel {
		Channel::A6 => ChannelMap::ChMap3,
		Channel::A7 => ChannelMap::ChMap2,
		Channel::A8 => ChannelMap::ChMap1,
		Channel::A9 => ChannelMap::ChMap0,
		Channel::A10 => ChannelMap::TcMap,
		Channel::A11 => ChannelMap::BatMap,
		Channel::Temperature => ChannelMap::TcMap,
		Channel::Battery => ChannelMap::BatMap,
	}
}

//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
