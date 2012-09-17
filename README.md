runits
======

This is a library for the Rust programming language that allows you to do math with 
floating point numbers together with units. For example, it allows you to divide
10 meters by 2 seconds yielding 5 meter/second. The library provides two types, methods
on those types, and a handful of functions.

Unit
--------
This is an enum with constructors for the [base SI units](http://en.wikipedia.org/wiki/SI),
[other SI units](http://en.wikipedia.org/wiki/Non-SI_units_accepted_for_use_with_SI),
[USA customary units](http://en.wikipedia.org/wiki/United_States_customary_units), and others
(`LightYear`, `Parsec`, `Packet`, `Bit`, and `Byte`). 

Compound units can be constructed with the Compound constructor: `Compound(@[Meter], @[Second])`
or more typically using multiplication and division: `Meter/Second`. Dimensionless numbers are
represented using unit `Compound(@[], @[])`.

All of the [SI prefixes](http://en.wikipedia.org/wiki/Metric_prefix) (e.g. Kilo and Micro) are included as well as 
the [IEC binary prefixes](http://en.wikipedia.org/wiki/Binary_prefix) (e.g. Kibi and Mebi). The binary prefixes
are base 1024 and typically only used to measure computer RAM; other quantities like disk space and
bandwidth should use the SI Prefixes.

Value
--------
Value is a struct with a float and a Unit. It supports all the usual operators, allows Values
to be converted from one (comaptible) unit to another, and provides normalize functions
which add prefixes like Kilo or Milli in order to make the value nice.

Invalid operations (e.g. adding a length to a time) cause the task to fail.

Usage
--------
	let speed = from_units(30.0, Mile/Hour);
	let delta = from_units(2.0, Meter/Second);
	let sum = speed + delta;   // for binary ops the rhs is converted to the lhs units
	
	let sum = sum.convert_to(Kilo*Meter/Second);
	info!("speed1 = %s", sum.to_str()); // prints "0.015411 km/s"
	
	let sum = sum.normalize_si();
	info!("speed2 = %s", sum.to_str()); // prints "15.411200 m/s"
	