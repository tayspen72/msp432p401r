//==============================================================================
// Notes
//==============================================================================
// mcu::gpio.rs
// Basic control over gpio pins

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use cortex_m_semihosting::hprintln;
use crate::mcu;
use msp432p401r;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinDirection{
	Input = 0,
	Output = 1
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinPull{
	PullUp = 1,
	PullDown = 0,
	PullDisabled = 2
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum PinState{
	PinLow = 0,
	PinHigh = 1
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub struct PinConfig{
	pub port: mcu::Port,
	pub pin: u8,
	pub direction: PinDirection,
	pub pull: PinPull,
	pub state: PinState,
}

//==============================================================================
// Variables
//==============================================================================
static DIO_HANDLE: Mutex<RefCell<Option<msp432p401r::DIO>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(dio: msp432p401r::DIO){
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| DIO_HANDLE.borrow(cs).replace(Some(dio)));

	unsafe {
		INITIALIZED = true;
	}
}

#[allow(dead_code)]
pub fn get_pin_state(config: &PinConfig) -> PinState {
	unsafe { if !INITIALIZED {
		return PinState::PinLow;
	}}
	
	let read = free(|cs|
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			match config.port {
				mcu::Port::Port1	=> dio.pain.read().p1in().bits(),
				mcu::Port::Port2	=> dio.pain.read().p2in().bits(),
				mcu::Port::Port3	=> dio.pbin.read().p3in().bits(),
				mcu::Port::Port4	=> dio.pbin.read().p4in().bits(),
				mcu::Port::Port5	=> dio.pcin.read().p5in().bits(),
				mcu::Port::Port6	=> dio.pcin.read().p6in().bits(),
				mcu::Port::Port7	=> dio.pdin.read().p7in().bits(),
				mcu::Port::Port8	=> dio.pdin.read().p8in().bits(),
				mcu::Port::Port9	=> dio.pein.read().p9in().bits(),
				mcu::Port::Port10 	=> dio.pein.read().p10in().bits(),
				mcu::Port::PortJ	=> (dio.pjin.read().pjin().bits() & 0xFF) as u8,
			}
		}
		else {
			0
		}
	);
	
	match read & (1 << config.pin) {
		0 => PinState::PinLow,
		_ => PinState::PinHigh
	}
}

#[allow(dead_code)]
pub fn pin_disable(_config: &PinConfig) {
	unsafe { if !INITIALIZED {
		return;
	}}
}

#[allow(dead_code)]
pub fn pin_setup(config: &PinConfig){
	unsafe { if !INITIALIZED {
		return;
	}}	
	
	let mut state: PinState = config.state;
	
	free(|cs| {
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			if let PinDirection::Input = config.direction {
				// Set pin direction
				match config.port {
					mcu::Port::Port1 => dio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits() | 0 << config.pin) }),
					mcu::Port::Port2 => dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 0 << config.pin) }),
					mcu::Port::Port3 => dio.pbdir.modify(|r, w| unsafe { w.p3dir().bits(r.p3dir().bits() | 0 << config.pin) }),
					mcu::Port::Port4 => dio.pbdir.modify(|r, w| unsafe { w.p4dir().bits(r.p4dir().bits() | 0 << config.pin) }),
					mcu::Port::Port5 => dio.pcdir.modify(|r, w| unsafe { w.p5dir().bits(r.p5dir().bits() | 0 << config.pin) }),
					mcu::Port::Port6 => dio.pcdir.modify(|r, w| unsafe { w.p6dir().bits(r.p6dir().bits() | 0 << config.pin) }),
					mcu::Port::Port7 => dio.pddir.modify(|r, w| unsafe { w.p7dir().bits(r.p7dir().bits() | 0 << config.pin) }),
					mcu::Port::Port8 => dio.pddir.modify(|r, w| unsafe { w.p8dir().bits(r.p8dir().bits() | 0 << config.pin) }),
					mcu::Port::Port9 => dio.pedir.modify(|r, w| unsafe { w.p9dir().bits(r.p9dir().bits() | 0 << config.pin) }),
					mcu::Port::Port10 => dio.pedir.modify(|r, w| unsafe { w.p10dir().bits(r.p10dir().bits() | 0 << config.pin) }),
					mcu::Port::PortJ => dio.pjdir.modify(|r, w| unsafe { w.pjdir().bits(r.pjdir().bits() | (0 << config.pin)) })
				}

				// Set pin pull as needed
				let pull = config.pull as u8;
				match config.port {
					mcu::Port::Port1 => dio.paren.modify(|r, w| unsafe { w.p1ren().bits(r.p1ren().bits() | pull << config.pin) }),
					mcu::Port::Port2 => dio.paren.modify(|r, w| unsafe { w.p2ren().bits(r.p2ren().bits() | pull << config.pin) }),
					mcu::Port::Port3 => dio.pbren.modify(|r, w| unsafe { w.p3ren().bits(r.p3ren().bits() | pull << config.pin) }),
					mcu::Port::Port4 => dio.pbren.modify(|r, w| unsafe { w.p4ren().bits(r.p4ren().bits() | pull << config.pin) }),
					mcu::Port::Port5 => dio.pcren.modify(|r, w| unsafe { w.p5ren().bits(r.p5ren().bits() | pull << config.pin) }),
					mcu::Port::Port6 => dio.pcren.modify(|r, w| unsafe { w.p6ren().bits(r.p6ren().bits() | pull << config.pin) }),
					mcu::Port::Port7 => dio.pdren.modify(|r, w| unsafe { w.p7ren().bits(r.p7ren().bits() | pull << config.pin) }),
					mcu::Port::Port8 => dio.pdren.modify(|r, w| unsafe { w.p8ren().bits(r.p8ren().bits() | pull << config.pin) }),
					mcu::Port::Port9 => dio.peren.modify(|r, w| unsafe { w.p9ren().bits(r.p9ren().bits() | pull << config.pin) }),
					mcu::Port::Port10 => dio.peren.modify(|r, w| unsafe { w.p10ren().bits(r.p10ren().bits() | pull << config.pin) }),
					mcu::Port::PortJ => dio.pjren.modify(|r, w| unsafe { w.pjren().bits(r.pjren().bits() | ((pull as u16) << config.pin)) })
				}

				// Set the pull state based on the otuput register value
				state = if let PinPull::PullUp = config.pull { PinState::PinHigh } else { PinState::PinLow };
			}
			else {
				// Set pin direction
				match config.port {
					mcu::Port::Port1 => dio.padir.modify(|r, w| unsafe { w.p1dir().bits(r.p1dir().bits() | 1 << config.pin) }),
					mcu::Port::Port2 => dio.padir.modify(|r, w| unsafe { w.p2dir().bits(r.p2dir().bits() | 1 << config.pin) }),
					mcu::Port::Port3 => dio.pbdir.modify(|r, w| unsafe { w.p3dir().bits(r.p3dir().bits() | 1 << config.pin) }),
					mcu::Port::Port4 => dio.pbdir.modify(|r, w| unsafe { w.p4dir().bits(r.p4dir().bits() | 1 << config.pin) }),
					mcu::Port::Port5 => dio.pcdir.modify(|r, w| unsafe { w.p5dir().bits(r.p5dir().bits() | 1 << config.pin) }),
					mcu::Port::Port6 => dio.pcdir.modify(|r, w| unsafe { w.p6dir().bits(r.p6dir().bits() | 1 << config.pin) }),
					mcu::Port::Port7 => dio.pddir.modify(|r, w| unsafe { w.p7dir().bits(r.p7dir().bits() | 1 << config.pin) }),
					mcu::Port::Port8 => dio.pddir.modify(|r, w| unsafe { w.p8dir().bits(r.p8dir().bits() | 1 << config.pin) }),
					mcu::Port::Port9 => dio.pedir.modify(|r, w| unsafe { w.p9dir().bits(r.p9dir().bits() | 1 << config.pin) }),
					mcu::Port::Port10 => dio.pedir.modify(|r, w| unsafe { w.p10dir().bits(r.p10dir().bits() | 1 << config.pin) }),
					mcu::Port::PortJ => dio.pjdir.modify(|r, w| unsafe { w.pjdir().bits(r.pjdir().bits() | (1 << config.pin)) })
				}
			}
		}
	});
	
	// Set the pin state after this critical section is left
	set_pin_state(config, state);
}

#[allow(dead_code)]
pub fn set_pin_state(config: &PinConfig, state: PinState){
	unsafe { if !INITIALIZED {
		return;
	}}
	
	free(|cs| {
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			let out = state as u8;
			match config.port {
				mcu::Port::Port1 => dio.paout.modify(|r, w| unsafe { w.p1out().bits(r.p1out().bits() | out << config.pin) }),
				mcu::Port::Port2 => dio.paout.modify(|r, w| unsafe { w.p2out().bits(r.p2out().bits() | out << config.pin) }),
				mcu::Port::Port3 => dio.pbout.modify(|r, w| unsafe { w.p3out().bits(r.p3out().bits() | out << config.pin) }),
				mcu::Port::Port4 => dio.pbout.modify(|r, w| unsafe { w.p4out().bits(r.p4out().bits() | out << config.pin) }),
				mcu::Port::Port5 => dio.pcout.modify(|r, w| unsafe { w.p5out().bits(r.p5out().bits() | out << config.pin) }),
				mcu::Port::Port6 => dio.pcout.modify(|r, w| unsafe { w.p6out().bits(r.p6out().bits() | out << config.pin) }),
				mcu::Port::Port7 => dio.pdout.modify(|r, w| unsafe { w.p7out().bits(r.p7out().bits() | out << config.pin) }),
				mcu::Port::Port8 => dio.pdout.modify(|r, w| unsafe { w.p8out().bits(r.p8out().bits() | out << config.pin) }),
				mcu::Port::Port9 => dio.peout.modify(|r, w| unsafe { w.p9out().bits(r.p9out().bits() | out << config.pin) }),
				mcu::Port::Port10 => dio.peout.modify(|r, w| unsafe { w.p10out().bits(r.p10out().bits() | out << config.pin) }),
				mcu::Port::PortJ => dio.pjout.modify(|r, w| unsafe { w.pjout().bits(r.pjout().bits() | ((out as u16) << config.pin)) })
			}
		}
	});
}

#[allow(dead_code)]
pub fn set_pin_function_select(config: &PinConfig, function: u8){
	unsafe { if !INITIALIZED {
		return;
	}}

	free(|cs| {
		if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
			let sel0 = if (function & 0x1) > 0 { 1 } else { 0 };
			let sel1 = if (function & 0x2) > 0 { 1 } else { 0 };

			match config.port {
				mcu::Port::Port1 => {
					dio.pasel0.modify(|_, w| unsafe { w.p1sel0().bits(sel0 << config.pin) });
					dio.pasel1.modify(|_, w| unsafe { w.p1sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port2 => {
					dio.pasel0.modify(|_, w| unsafe { w.p2sel0().bits(sel0 << config.pin) });
					dio.pasel1.modify(|_, w| unsafe { w.p2sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port3 => {
					dio.pbsel0.modify(|_, w| unsafe { w.p3sel0().bits(sel0 << config.pin) });
					dio.pbsel1.modify(|_, w| unsafe { w.p3sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port4 => {
					dio.pbsel0.modify(|_, w| unsafe { w.p4sel0().bits(sel0 << config.pin) });
					dio.pbsel1.modify(|_, w| unsafe { w.p4sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port5 => {
					dio.pcsel0.modify(|_, w| unsafe { w.p5sel0().bits(sel0 << config.pin) });
					dio.pcsel1.modify(|_, w| unsafe { w.p5sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port6 => {
					dio.pcsel0.modify(|_, w| unsafe { w.p6sel0().bits(sel0 << config.pin) });
					dio.pcsel1.modify(|_, w| unsafe { w.p6sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port7 => {
					dio.pdsel0.modify(|_, w| unsafe { w.p7sel0().bits(sel0 << config.pin) });
					dio.pdsel1.modify(|_, w| unsafe { w.p7sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port8 => {
					dio.pdsel0.modify(|_, w| unsafe { w.p8sel0().bits(sel0 << config.pin) });
					dio.pdsel1.modify(|_, w| unsafe { w.p8sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port9 => {
					dio.pesel0.modify(|_, w| unsafe { w.p9sel0().bits(sel0 << config.pin) });
					dio.pesel1.modify(|_, w| unsafe { w.p9sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::Port10 => {
					dio.pesel0.modify(|_, w| unsafe { w.p10sel0().bits(sel0 << config.pin) });
					dio.pesel1.modify(|_, w| unsafe { w.p10sel1().bits(sel1 << config.pin) });
				},
				mcu::Port::PortJ => {
					dio.pasel0.modify(|_, w| unsafe { w.p1sel0().bits(sel0 << config.pin) });
					dio.pasel1.modify(|_, w| unsafe { w.p1sel1().bits(sel1 << config.pin) });
				},
			}
		}
	});
}

#[allow(dead_code)]
pub fn print_in(port: mcu::Port) {
	hprintln!("din{}: {}", port as u8, 
		free(|cs|
			if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match port {
					mcu::Port::Port1	=> dio.pain.read().p1in().bits(),
					mcu::Port::Port2	=> dio.pain.read().p2in().bits(),
					mcu::Port::Port3	=> dio.pbin.read().p3in().bits(),
					mcu::Port::Port4	=> dio.pbin.read().p4in().bits(),
					mcu::Port::Port5	=> dio.pcin.read().p5in().bits(),
					mcu::Port::Port6	=> dio.pcin.read().p6in().bits(),
					mcu::Port::Port7	=> dio.pdin.read().p7in().bits(),
					mcu::Port::Port8	=> dio.pdin.read().p8in().bits(),
					mcu::Port::Port9	=> dio.pein.read().p9in().bits(),
					mcu::Port::Port10 	=> dio.pein.read().p10in().bits(),
					mcu::Port::PortJ	=> (dio.pjin.read().pjin().bits() & 0xFF) as u8,
				}
			}
			else {
				0
			}
		)
	).unwrap();	
}

#[allow(dead_code)]
pub fn print_out(port: mcu::Port) {
	hprintln!("dout{}: {}", port as u8, 
		free(|cs|
			if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match port {
					mcu::Port::Port1	=> dio.paout.read().p1out().bits(),
					mcu::Port::Port2	=> dio.paout.read().p2out().bits(),
					mcu::Port::Port3	=> dio.pbout.read().p3out().bits(),
					mcu::Port::Port4	=> dio.pbout.read().p4out().bits(),
					mcu::Port::Port5	=> dio.pcout.read().p5out().bits(),
					mcu::Port::Port6	=> dio.pcout.read().p6out().bits(),
					mcu::Port::Port7	=> dio.pdout.read().p7out().bits(),
					mcu::Port::Port8	=> dio.pdout.read().p8out().bits(),
					mcu::Port::Port9	=> dio.peout.read().p9out().bits(),
					mcu::Port::Port10 	=> dio.peout.read().p10out().bits(),
					mcu::Port::PortJ	=> (dio.pjout.read().pjout().bits() & 0xFF) as u8,
				}
			}
			else {
				0
			}
		)
	).unwrap();	
}

#[allow(dead_code)]
pub fn print_dir(port: mcu::Port) {
	hprintln!("dir{}: {}", port as u8, 
		free(|cs|
			if let Some(ref mut dio) = DIO_HANDLE.borrow(cs).borrow_mut().deref_mut() {
				match port {
					mcu::Port::Port1	=> dio.padir.read().p1dir().bits(),
					mcu::Port::Port2	=> dio.padir.read().p2dir().bits(),
					mcu::Port::Port3	=> dio.pbdir.read().p3dir().bits(),
					mcu::Port::Port4	=> dio.pbdir.read().p4dir().bits(),
					mcu::Port::Port5	=> dio.pcdir.read().p5dir().bits(),
					mcu::Port::Port6	=> dio.pcdir.read().p6dir().bits(),
					mcu::Port::Port7	=> dio.pddir.read().p7dir().bits(),
					mcu::Port::Port8	=> dio.pddir.read().p8dir().bits(),
					mcu::Port::Port9	=> dio.pedir.read().p9dir().bits(),
					mcu::Port::Port10 	=> dio.pedir.read().p10dir().bits(),
					mcu::Port::PortJ	=> (dio.pjdir.read().pjdir().bits() & 0xFF) as u8,
				}
			}
			else {
				0
			}
		)
	).unwrap();	
}

//==============================================================================
// Private Functions
//==============================================================================


//==============================================================================
// Interrupt Handler
//==============================================================================


//==============================================================================
// Task Handler
//==============================================================================
