//==============================================================================
// Notes
//==============================================================================
// mcu::eusci.rs
// Provedes access and handles for the eUSCI peripheral objects

//==============================================================================
// Crates and Mods
//==============================================================================
use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use crate::mcu;
use msp432p401r_pac;

//==============================================================================
// Enums, Structs, and Types
//==============================================================================
#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub enum EUSCI{
	A0,
	A1,
	A2,
	A3,
	B0,
	B1,
	B2,
	B3,
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq)]
pub struct I2C{
	pub sda_port: mcu::Port,
	pub sda_pin: u8,
	pub scl_port: mcu::Port,
	pub scl_pin: u8,
	pub eusci: EUSCI,
	pub address: u8,
	pub speed: u32
}

//==============================================================================
// Variables
//==============================================================================
static EUSCI_A0_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::EUSCI_A0>>> = 
	Mutex::new(RefCell::new(None));
static EUSCI_A1_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::EUSCI_A1>>> = 
	Mutex::new(RefCell::new(None));
static EUSCI_A2_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::EUSCI_A2>>> = 
	Mutex::new(RefCell::new(None));
static EUSCI_A3_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::EUSCI_A3>>> = 
	Mutex::new(RefCell::new(None));
static EUSCI_B0_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::EUSCI_B0>>> = 
	Mutex::new(RefCell::new(None));
static EUSCI_B1_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::EUSCI_B1>>> = 
	Mutex::new(RefCell::new(None));
static EUSCI_B2_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::EUSCI_B2>>> = 
	Mutex::new(RefCell::new(None));
static EUSCI_B3_HANDLE: Mutex<RefCell<Option<msp432p401r_pac::EUSCI_B3>>> = 
	Mutex::new(RefCell::new(None));

static mut INITIALIZED: bool = false;

//==============================================================================
// Public Functions
//==============================================================================
#[allow(dead_code)]
pub fn init(
	eusci_a0: msp432p401r_pac::EUSCI_A0,
	eusci_a1: msp432p401r_pac::EUSCI_A1,
	eusci_a2: msp432p401r_pac::EUSCI_A2,
	eusci_a3: msp432p401r_pac::EUSCI_A3,
	eusci_b0: msp432p401r_pac::EUSCI_B0,
	eusci_b1: msp432p401r_pac::EUSCI_B1,
	eusci_b2: msp432p401r_pac::EUSCI_B2,
	eusci_b3: msp432p401r_pac::EUSCI_B3,
){
	unsafe { if INITIALIZED {
		return;
	}}

	free(|cs| EUSCI_A0_HANDLE.borrow(cs).replace(Some(eusci_a0)));
	free(|cs| EUSCI_A1_HANDLE.borrow(cs).replace(Some(eusci_a1)));
	free(|cs| EUSCI_A2_HANDLE.borrow(cs).replace(Some(eusci_a2)));
	free(|cs| EUSCI_A3_HANDLE.borrow(cs).replace(Some(eusci_a3)));
	free(|cs| EUSCI_B0_HANDLE.borrow(cs).replace(Some(eusci_b0)));
	free(|cs| EUSCI_B1_HANDLE.borrow(cs).replace(Some(eusci_b1)));
	free(|cs| EUSCI_B2_HANDLE.borrow(cs).replace(Some(eusci_b2)));
	free(|cs| EUSCI_B3_HANDLE.borrow(cs).replace(Some(eusci_b3)));

	unsafe {
		INITIALIZED = true;
	}
}

pub fn i2c_init(i2c: &I2C){
	free(|cs| {
		match i2c.eusci {
			EUSCI::A0 => (),
			EUSCI::A1 => (),
			EUSCI::A2 => (),
			EUSCI::A3 => (),
			EUSCI::B0 => {
				if let Some(ref mut eusci) = EUSCI_B0_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					// Assert SWRST bit during config
					eusci.ucbx_ctlw0.write(|w| w.ucswrst().set_bit());
					
					eusci.ucbx_ctlw0.modify(|_, w| w
						.ucssel().ucssel_2()
						.ucsync().set_bit()
						.ucmst().set_bit()
						.ucmm().clear_bit()
						.ucsla10().clear_bit()
						.uca10().clear_bit()
					);

					// Determine baud rate values
					let brw = mcu::get_system_clock().sm_clk / i2c.speed;
					eusci.ucbx_brw.write(|w| unsafe { w.ucbr().bits(brw as u16) });
					
					// Release the bit to use this config
					eusci.ucbx_ctlw0.modify(|_, w| w.ucswrst().set_bit());
				}
			},
			EUSCI::B1 => {
				if let Some(ref mut eusci) = EUSCI_B1_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					
				}
			},
			EUSCI::B2 => {
				if let Some(ref mut eusci) = EUSCI_B2_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					
				}
			},
			EUSCI::B3 => {
				if let Some(ref mut eusci) = EUSCI_B3_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					
				}
			},
		}
	});
}

pub fn i2c_write_block(i2c: &I2C, data: &[u8], send_stop: bool){
	free(|cs| {
		match i2c.eusci {
			EUSCI::A0 => (),
			EUSCI::A1 => (),
			EUSCI::A2 => (),
			EUSCI::A3 => (),
			EUSCI::B0 => {
				if let Some(ref mut eusci) = EUSCI_B0_HANDLE.borrow(cs).borrow_mut().deref_mut() {
					// Set to transmitter mode
					eusci.ucbx_ctlw0.modify(|_, w| w.uctr().set_bit());

					// Send start condition and address
					eusci.ucbx_ctlw0.modify(|_, w| w.uctxstt().set_bit());

					// Wait for bus to be ready for transmit
					while eusci.ucbx_ifg.read().uctxifg0().is_uctxifg0_0() {}

					if let Some((last, buf)) = data.split_last() {
						if !buf.is_empty() {
							for byte in buf.into_iter() {
								// Load byte into transmit buffer
								eusci.ucbx_txbuf.write(|w| unsafe { w.uctxbuf().bits(*byte) });

								// Wait for transmit
								while eusci.ucbx_ifg.read().uctxifg0().is_uctxifg0_0() {}
							}

							// Send last byte
							eusci.ucbx_txbuf.write(|w| unsafe { w.uctxbuf().bits(*last) });

							// Send stop condition on next transmission if needed
							if send_stop {
								eusci.ucbx_ctlw0.modify(|_, w| w.uctxstp().set_bit());
							}

							// Wait for transmission to stop
							
						}
					}
				}
			},
			EUSCI::B1 => (),
			EUSCI::B2 => (),
			EUSCI::B3 => (),
		}

	});
}

pub fn i2c_read_block(i2c: &I2C, data: &[u8], send_stop: bool){

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
