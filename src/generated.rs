// DO NOT EDIT: generated from units.in using gen_units.py
export Unit, Meter, Feet, Micro, Milli, Kilo, Second, Minute, Compound;
export canonical_unit, is_modifier, unit_type, unit_abrev;	// these are really internal items

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

pure fn canonical_unit(u: Unit) -> (float, @[Unit])
{
	match u
	{
		// length
		Meter			=> (1.0, @[Meter]),
		Feet			=> (0.3048, @[Meter]),
		
		// time
		Second			=> (1.0, @[Second]),
		Minute			=> (60.0, @[Second]),
		
		// modifiers
		Micro			=> (1.0e-6, @[]),
		Milli			=> (1.0e-3, @[]),
		Kilo			=> (1.0e3, @[]),
		
		// compound
		Compound(*)	=> fail fmt!("Expected a simple unit but found %?", u),
	}
}

pure fn is_modifier(u: Unit) -> bool
{
	match u
	{
		Micro | Milli | Kilo => true,
		_ => false,
	}
}

pure fn unit_type(u: Unit) -> ~str
{
	match u
	{
		Meter | Feet => ~"length",
		Micro | Milli | Kilo => ~"",
		Second | Minute => ~"time",
		Compound(*) => fail fmt!("unit_type should only be called with simple units, not %?", u),
	}
}

pure fn unit_abrev(u: Unit) -> ~str
{
	match u
	{
		// length
		Meter => ~"m",
		Feet => ~"ft",
		
		// modifiers
		Micro => ~"u",
		Milli => ~"m",
		Kilo => ~"k",
		
		// time
		Second => ~"s",
		Minute => ~"min",
		
		// compound
		Compound(*)	=> fail fmt!("unit_abrev should only be called with simple units, not %?", u),
	}
}
