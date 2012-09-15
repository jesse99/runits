// DO NOT EDIT: generated from units.in using gen_units.py
export Unit, Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci, Hecto, Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta, Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Kelvin, Celsius, Fahrenheit, Meter, AU, Inch, Feet, Yard, Mile, NauticalMile, Gram, Tonne, Dram, Ounce, Pound, Second, Minute, Hour, Day, Mole, Candela, Ampere, Compound;
export canonical_unit, is_modifier, unit_type, unit_abrev;	// these are really internal items

/// Simple units are specified with one of the constructors (e.g. Meter).
/// Compound units are constructed using multiplication and division
/// (e.g. Meter/(Second*Second)). Dimensionless units are empty Compound
/// units.
enum Unit
{
	// modifiers
	Yocto,
	Zepto,
	Atto,
	Femto,
	Pico,
	Nano,
	Micro,
	Milli,
	Centi,
	Deci,
	Hecto,
	Kilo,
	Mega,
	Giga,
	Tera,
	Peta,
	Exa,
	Zetta,
	Yotta,
	Kibi,
	Mebi,
	Gibi,
	Tebi,
	Pebi,
	Exbi,
	
	// temperature
	Kelvin,
	Celsius,
	Fahrenheit,
	
	// length
	Meter,
	AU,
	Inch,
	Feet,
	Yard,
	Mile,
	NauticalMile,
	
	// mass
	Gram,
	Tonne,
	Dram,
	Ounce,
	Pound,
	
	// time
	Second,
	Minute,
	Hour,
	Day,
	
	// amount_of_substance
	Mole,
	
	// luminous_intensity
	Candela,
	
	// electric_current
	Ampere,
	
	// compound
	Compound(@[Unit], @[Unit]),	// numerator, denominator (which must be simple units)
}

pure fn canonical_unit(u: Unit) -> (float, @[Unit])
{
	match u
	{
		// temperature
		Kelvin			=> (1.0, @[Kelvin]),
		Celsius			=> (1.0, @[Kelvin]),
		Fahrenheit			=> (1.0, @[Kelvin]),
		
		// length
		Meter			=> (1.0, @[Meter]),
		AU			=> (1.49597870691e11, @[Meter]),
		Inch			=> (0.0254, @[Meter]),
		Feet			=> (0.3048, @[Meter]),
		Yard			=> (0.9144, @[Meter]),
		Mile			=> (1609.344, @[Meter]),
		NauticalMile			=> (1852.0, @[Meter]),
		
		// mass
		Gram			=> (1.0, @[Gram]),
		Tonne			=> (1.0e6, @[Gram]),
		Dram			=> (1.7718451953, @[Gram]),
		Ounce			=> (28.349523125, @[Gram]),
		Pound			=> (453.59237, @[Gram]),
		
		// time
		Second			=> (1.0, @[Second]),
		Minute			=> (60.0, @[Second]),
		Hour			=> (3600.0, @[Second]),
		Day			=> (86400.0, @[Second]),
		
		// amount_of_substance
		Mole			=> (1.0, @[Mole]),
		
		// luminous_intensity
		Candela			=> (1.0, @[Candela]),
		
		// electric_current
		Ampere			=> (1.0, @[Ampere]),
		
		// modifiers
		Yocto			=> (1.0e-24, @[]),
		Zepto			=> (1.0e-21, @[]),
		Atto			=> (1.0e-18, @[]),
		Femto			=> (1.0e-15, @[]),
		Pico			=> (1.0e-12, @[]),
		Nano			=> (1.0e-9, @[]),
		Micro			=> (1.0e-6, @[]),
		Milli			=> (1.0e-3, @[]),
		Centi			=> (1.0e-2, @[]),
		Deci			=> (1.0e-1, @[]),
		Hecto			=> (1.0e2, @[]),
		Kilo			=> (1.0e3, @[]),
		Mega			=> (1.0e6, @[]),
		Giga			=> (1.0e9, @[]),
		Tera			=> (1.0e12, @[]),
		Peta			=> (1.0e15, @[]),
		Exa			=> (1.0e18, @[]),
		Zetta			=> (1.0e21, @[]),
		Yotta			=> (1.0e24, @[]),
		Kibi			=> (1024.0, @[]),
		Mebi			=> (1048576.0, @[]),
		Gibi			=> (1073741824.0, @[]),
		Tebi			=> (1099511627776.0, @[]),
		Pebi			=> (1125899906842624.0, @[]),
		Exbi			=> (1152921504606846976.0, @[]),
		
		// compound
		Compound(*)	=> fail fmt!("Expected a simple unit but found %?", u),
	}
}

pure fn is_modifier(u: Unit) -> bool
{
	match u
	{
		Yocto | Zepto | Atto | Femto | Pico | Nano | Micro | Milli | Centi | Deci | Hecto | Kilo | Mega | Giga | Tera | Peta | Exa | Zetta | Yotta | Kibi | Mebi | Gibi | Tebi | Pebi | Exbi => true,
		_ => false,
	}
}

pure fn unit_type(u: Unit) -> ~str
{
	match u
	{
		Yocto | Zepto | Atto | Femto | Pico | Nano | Micro | Milli | Centi | Deci | Hecto | Kilo | Mega | Giga | Tera | Peta | Exa | Zetta | Yotta | Kibi | Mebi | Gibi | Tebi | Pebi | Exbi => ~"",
		Kelvin | Celsius | Fahrenheit => ~"temperature",
		Meter | AU | Inch | Feet | Yard | Mile | NauticalMile => ~"length",
		Gram | Tonne | Dram | Ounce | Pound => ~"mass",
		Second | Minute | Hour | Day => ~"time",
		Mole => ~"amount_of_substance",
		Candela => ~"luminous_intensity",
		Ampere => ~"electric_current",
		Compound(*) => fail fmt!("unit_type should only be called with simple units, not %?", u),
	}
}

pure fn unit_abrev(u: Unit) -> ~str
{
	match u
	{
		// modifiers
		Yocto => ~"y",
		Zepto => ~"z",
		Atto => ~"a",
		Femto => ~"f",
		Pico => ~"p",
		Nano => ~"n",
		Micro => ~"u",
		Milli => ~"m",
		Centi => ~"c",
		Deci => ~"d",
		Hecto => ~"h",
		Kilo => ~"k",
		Mega => ~"M",
		Giga => ~"G",
		Tera => ~"T",
		Peta => ~"P",
		Exa => ~"E",
		Zetta => ~"Z",
		Yotta => ~"Y",
		Kibi => ~"Ki",
		Mebi => ~"Mi",
		Gibi => ~"Gi",
		Tebi => ~"Ti",
		Pebi => ~"Pi",
		Exbi => ~"Ei",
		
		// temperature
		Kelvin => ~"K",
		Celsius => ~"C",
		Fahrenheit => ~"F",
		
		// length
		Meter => ~"m",
		AU => ~"ua",
		Inch => ~"in",
		Feet => ~"ft",
		Yard => ~"yd",
		Mile => ~"mi",
		NauticalMile => ~"nmi",
		
		// mass
		Gram => ~"g",
		Tonne => ~"t",
		Dram => ~"dr",
		Ounce => ~"oz",
		Pound => ~"lb",
		
		// time
		Second => ~"s",
		Minute => ~"min",
		Hour => ~"h",
		Day => ~"d",
		
		// amount_of_substance
		Mole => ~"mol",
		
		// luminous_intensity
		Candela => ~"cd",
		
		// electric_current
		Ampere => ~"A",
		
		// compound
		Compound(*)	=> fail fmt!("unit_abrev should only be called with simple units, not %?", u),
	}
}
