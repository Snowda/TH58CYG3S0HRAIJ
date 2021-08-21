#![no_std]
/** Based on Kioxia TH58CYG3S0HRAIJ datasheet
* 21/08/2021 by Conor Forde <me@conorforde.com>
* Updates should be available at https://github.com/Snowda/TH58CYG3S0HRAIJ
*
* Changelog:
*     ... - ongoing development release

* NOTE: THIS IS ONLY A PARIAL RELEASE. 
* THIS DEVICE CLASS IS CURRENTLY UNDERGOING ACTIVE DEVELOPMENT AND IS MISSING MOST FEATURES. 
* PLEASE KEEP THIS IN MIND IF YOU DECIDE TO USE THIS PARTICULAR CODE FOR ANYTHING.
*/

extern crate embedded_hal;

use embedded_hal::blocking::spi::Transfer; // Write
use embedded_hal::digital::v2::{OutputPin, InputPin};
use embedded_hal::blocking::delay::DelayMs;

pub struct TH58CYG3S0HRAIJ {
    _address: u8,
}

impl TH58CYG3S0HRAIJ {
	fn init(&self) {
    	self._address= TH58CYG3S0HRAIJ_DEFAULT_ADDRESS;
	}

	fn writeRegister(&self, register_addr: u8, value: u8) -> bool {
	    //send write call to sensor address
	    //send register address to sensor
	    //send value to register
	    let write_status: bool = false;
	    //Wire.beginTransmission(_address); //open communication with 
	    //Wire.write(register_addr);  
	    //Wire.write(value); 
	    //Wire.endTransmission(); 

	    return write_status; //returns whether the write succeeded or failed
	}

	fn writeRegisters(&self, msb_register: u8, msb_value: u8, lsb_register: u8, lsb_value: u8) -> bool { 
	    //send write call to sensor address
	    //send register address to sensor
	    //send value to register
	    let msb_bool: bool = writeRegister(msb_register, msb_value);
	    let lsb_bool: bool = writeRegister(lsb_register, lsb_value);
	    return false; //returns whether the write succeeded or failed
	}

	fn writeMaskedRegister(&self, register_addr: u8, mask: u8, value: u8) -> bool {
	    let masked_value: u8 = (mask & value); //there has to be an easier way to do this.... I know, I know, shut up, I know it's that, I'll get around to it when I can ok?
	    return writeRegister(register_addr, masked_value);
	    //every reference to this is wrong (also)!! fix them!
	}

	fn readRegister(&self, register_addr: u8) -> u8 {
	    //call sensor by address
	    //call registers
	    let data: u8 = 0;

	    //Wire.beginTransmission(_address); 
	    //Wire.write(register_addr); 
	    //Wire.endTransmission(); 

	    //Wire.requestFrom((int)_address, 1);

	    //while(Wire.available()) {
	    //    data = Wire.read();    // receive a byte as character
	    //}

	    return data; //return the data returned from the register
	}

	fn readRegisters(&self, msb_register: u8, lsb_register: u8) -> u16 {
	    let msb: u8 = readRegister(msb_register);
	    let lsb: u8 = readRegister(lsb_register);
	    return ((msb as u16) << 8) | lsb;

	}

	fn readMaskedRegister(&self, register_addr: u8, mask: u8) -> u8 {
	    let data: u8 = self.readRegister(register_addr);
	    return (data & mask);

	    //every reference to this is wrong!!! fix them
	}

	fn readPins(&self) -> u8 {
	    return self.readRegister(TCA6408A_OUTPUT);
	}

	fn readOutput(&self, port_number: u8) -> bool {
	    return self.readMaskedRegister(TCA6408A_OUTPUT, (0xFE >> port_number)) != 0;
	}

	fn readInput(&self, port_number: u8) -> bool {
	    return self.readMaskedRegister(TCA6408A_INPUT, (0x01 << port_number)) != 0;
	}

	fn setInput(&self, port_number: u8) {
	    uint8_t config = self.readRegister(TCA6408A_CONFIG);
	    if port_number < 8 {
	        writeRegister(config || (0x01 << port_number))
	    }
	}

	fn setOutput(&self, port_number: u8) {
	    let config: u8 = readRegister(TCA6408A_CONFIG);
	    if port_number < 8 {
	        self.writeRegister(config && (0xFE << port_number))
	    }
	}
}