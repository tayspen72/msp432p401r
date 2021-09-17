//==============================================================================
// Notes
//==============================================================================
// config.rs

//==============================================================================
// Crates and Mods
//==============================================================================
use crate::mcu;
use crate::mcu::{adc, counter, eusci};

//==============================================================================
// ADC
//==============================================================================


//==============================================================================
// Button
//==============================================================================
#[allow(dead_code)] pub const BUTTON_1_PORT: mcu::Port = mcu::Port::Port1;
#[allow(dead_code)] pub const BUTTON_1_PIN: u8 = 1;
#[allow(dead_code)] pub const BUTTON_2_PORT: mcu::Port = mcu::Port::Port1;
#[allow(dead_code)] pub const BUTTON_2_PIN: u8 = 4;

//==============================================================================
// Clock
//==============================================================================
#[allow(dead_code)] pub const LFXCLK_IN_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const LFXCLK_IN_PIN: u8 = 0;
#[allow(dead_code)] pub const LFXCLK_OUT_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const LFXCLK_OUT_PIN: u8 = 1;
#[allow(dead_code)] pub const HFXCLK_IN_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const HFXCLK_IN_PIN: u8 = 2;
#[allow(dead_code)] pub const HFXCLK_OUT_PORT: mcu::Port = mcu::Port::PortJ;
#[allow(dead_code)] pub const HFXCLK_OUT_PIN: u8 = 3;

//==============================================================================
// Counter
//==============================================================================
#[allow(dead_code)] pub const COUNTER_TACLK_PORT: mcu::Port = mcu::Port::Port4;
#[allow(dead_code)] pub const COUNTER_TACLK_PIN: u8 = 2;
#[allow(dead_code)] pub const COUNTER_TACLK: counter::TaClk = counter::TaClk::A2;
#[allow(dead_code)] pub const COUNTER_FUNCTION_SELECT: u8 = 0b10;

//==============================================================================
// Debug
//==============================================================================


//==============================================================================
// Flash
//==============================================================================


//==============================================================================
// I2C
//==============================================================================
#[allow(dead_code)] pub const I2C_SDA_PORT: mcu::Port = mcu::Port::Port1;
#[allow(dead_code)] pub const I2C_SDA_PIN: u8 = 6;
#[allow(dead_code)] pub const I2C_SCL_PORT: mcu::Port = mcu::Port::Port1;
#[allow(dead_code)] pub const I2C_SCL_PIN: u8 = 7;
#[allow(dead_code)] pub const I2C_EUSCI: eusci::EUSCI = eusci::EUSCI::B0;
#[allow(dead_code)] pub const I2C_ADDRESS: u8 = 0x70;
#[allow(dead_code)] pub const I2C_SPEED: u32 = 400_000;
#[allow(dead_code)] pub const I2C_FUNCTION_SELECT: u8 = 0b01;

//==============================================================================
// Input
//==============================================================================
#[allow(dead_code)] pub const INPUT_QUEUE_LENGTH: u8 = 16;


//==============================================================================
// Push Button
//==============================================================================
#[allow(dead_code)] pub const PUSH_BUTTON_IN_PIN: u8 	= 13;
#[allow(dead_code)] pub const PUSH_BUTTON_OUT_PIN: u8 	= 15;

//==============================================================================
// RTC
//==============================================================================


//==============================================================================
// SPI
//==============================================================================


//==============================================================================
// Temperature Sensor
//==============================================================================
#[allow(dead_code)] pub const TEMPERATURE_ADC_PORT: mcu::Port = mcu::Port::PortDisabled;
#[allow(dead_code)] pub const TEMPERATURE_ADC_PIN: u8 = 0;
#[allow(dead_code)] pub const TEMPERATURE_ADC_CHANNEL: adc::Channel = adc::Channel::Temperature;
#[allow(dead_code)] pub const TEMPERATURE_ADC_SIGNAL: u8 = 0;
#[allow(dead_code)] pub const TEMPERATURE_ADC_FUNCTION_SELECT: u8 = 0b11;

//==============================================================================
// Uart
//===================i===========================================================
