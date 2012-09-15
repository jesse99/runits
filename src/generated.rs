// DO NOT EDIT: generated from units.in using gen_units.py
export Unit, Meter, Feet, Micro, Milli, Kilo, Second, Minute, Compound;

/// Simple units are specified with one of the constructors (e.g. Meter).
/// Compound units are constructed using multiplication and division
/// (e.g. Meter/(Second*Second)). Dimensionless units are empty Compound
/// units.
enum Unit
{
	// length
	Meter,
	Feet,
	
	// modifiers
	Micro,
	Milli,
	Kilo,
	
	// time
	Second,
	Minute,
	
	// compound
	Compound(@[Unit], @[Unit]),	// numerator, denominator (which must be simple units)
}
