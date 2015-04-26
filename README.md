TCA6408A
======================

Device driver for TCA6408A IO Expander

![TCA6408A datasheet (PDF)](http://www.ti.com/lit/ds/symlink/tca6408a.pdf)

======================
Installation
======================

Requires the Arduino Libraries
![Wire.h](http://arduino.cc/en/reference/wire) 
and 
![SPI.h](http://arduino.cc/en/Reference/SPI) 
depending on how you wish to interface with the chip.

*warning* SPI not supported yet!

Download the repo as a zip file and install throught the Arduino IDE and select:

Sketch -> Import Library -> Add Library

Make sure that if you are installing updates that you remove any pre-existing libraries called "TCA6408A".

======================
Capibilities
======================

Sets IO pins as input or output pins and interfaces over I2C. Also contains interupt handling and reset capibilities


More to be documented

======================
Disclaimer:
======================

I was not expecting this much attention this early in developement honestly. 

A large portion of this code is incomplete and broken. For now I just ask to be either patient, or fork it. 

Also, while I will now be reviewing the license agreement, the end result will both be allowed for private and public usage, just don't hold me to any commercial support. 

With this in mind please submit any pull requests and I will glady accept them once tested as I do like to end up with good code.