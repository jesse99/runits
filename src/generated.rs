// DO NOT EDIT: generated from units.in using gen_units.py
export Unit, Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci, Hecto, Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta, Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Kelvin, Celsius, Fahrenheit, Hectare, Liter, Gallon, Meter, AU, Inch, Feet, Yard, Mile, NauticalMile, Gram, Tonne, Dram, Ounce, Pound, Second, Minute, Hour, Day, Month, Year, Mole, Candela, Bit, Byte, Ampere, Compound;
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
	
	// area
	Hectare,
	
	// volume
	Liter,
	Gallon,
	
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
	Month,
	Year,
	
	// amount_of_substance
	Mole,
	
	// luminous_intensity
	Candela,
	
	// bits
	Bit,
	Byte,
	
	// electric_current
	Ampere,
	
	// compound
	Compound(@[Unit], @[Unit]),	// numerator, denominator (which must be simple units)
}

pure fn canonical_unit(u: Unit) -> (float, float, @[Unit])
{
	match u
	{
		// temperature
		Kelvin			=> (0.0, 1.0, @[Kelvin]),
		Celsius			=> (273.15, 1.0, @[Kelvin]),
		Fahrenheit			=> (459.67, 0.55556, @[Kelvin]),
		
		// area
		Hectare			=> (0.0, 10000.0, @[Meter*Meter]),
		
		// volume
		Liter			=> (0.0, 0.001, @[Meter*Meter*Meter]),
		Gallon			=> (0.0, 0.003785411784, @[Meter*Meter*Meter]),
		
		// length
		Meter			=> (0.0, 1.0, @[Meter]),
		AU			=> (0.0, 1.49597870691e11, @[Meter]),
		Inch			=> (0.0, 0.0254, @[Meter]),
		Feet			=> (0.0, 0.3048, @[Meter]),
		Yard			=> (0.0, 0.9144, @[Meter]),
		Mile			=> (0.0, 1609.344, @[Meter]),
		NauticalMile			=> (0.0, 1852.0, @[Meter]),
		
		// mass
		Gram			=> (0.0, 1.0, @[Gram]),
		Tonne			=> (0.0, 1.0e6, @[Gram]),
		Dram			=> (0.0, 1.7718451953, @[Gram]),
		Ounce			=> (0.0, 28.349523125, @[Gram]),
		Pound			=> (0.0, 453.59237, @[Gram]),
		
		// time
		Second			=> (0.0, 1.0, @[Second]),
		Minute			=> (0.0, 60.0, @[Second]),
		Hour			=> (0.0, 3600.0, @[Second]),
		Day			=> (0.0, 86400.0, @[Second]),
		Month			=> (0.0, 2629746.0, @[Second]),
		Year			=> (0.0, 31557600.0, @[Second]),
		
		// amount_of_substance
		Mole			=> (0.0, 1.0, @[Mole]),
		
		// luminous_intensity
		Candela			=> (0.0, 1.0, @[Candela]),
		
		// bits
		Bit			=> (0.0, 1.0, @[Bit]),
		Byte			=> (0.0, 8.0, @[Bit]),
		
		// electric_current
		Ampere			=> (0.0, 1.0, @[Ampere]),
		
		// modifiers
		Yocto			=> (0.0, 1.0e-24, @[]),
		Zepto			=> (0.0, 1.0e-21, @[]),
		Atto			=> (0.0, 1.0e-18, @[]),
		Femto			=> (0.0, 1.0e-15, @[]),
		Pico			=> (0.0, 1.0e-12, @[]),
		Nano			=> (0.0, 1.0e-9, @[]),
		Micro			=> (0.0, 1.0e-6, @[]),
		Milli			=> (0.0, 1.0e-3, @[]),
		Centi			=> (0.0, 1.0e-2, @[]),
		Deci			=> (0.0, 1.0e-1, @[]),
		Hecto			=> (0.0, 1.0e2, @[]),
		Kilo			=> (0.0, 1.0e3, @[]),
		Mega			=> (0.0, 1.0e6, @[]),
		Giga			=> (0.0, 1.0e9, @[]),
		Tera			=> (0.0, 1.0e12, @[]),
		Peta			=> (0.0, 1.0e15, @[]),
		Exa			=> (0.0, 1.0e18, @[]),
		Zetta			=> (0.0, 1.0e21, @[]),
		Yotta			=> (0.0, 1.0e24, @[]),
		Kibi			=> (0.0, 1024.0, @[]),
		Mebi			=> (0.0, 1048576.0, @[]),
		Gibi			=> (0.0, 1073741824.0, @[]),
		Tebi			=> (0.0, 1099511627776.0, @[]),
		Pebi			=> (0.0, 1125899906842624.0, @[]),
		Exbi			=> (0.0, 1152921504606846976.0, @[]),
		
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
		Hectare => ~"area",
		Liter | Gallon => ~"volume",
		Meter | AU | Inch | Feet | Yard | Mile | NauticalMile => ~"length",
		Gram | Tonne | Dram | Ounce | Pound => ~"mass",
		Second | Minute | Hour | Day | Month | Year => ~"time",
		Mole => ~"amount_of_substance",
		Candela => ~"luminous_intensity",
		Bit | Byte => ~"bits",
		Ampere => ~"electric_current",
		Compound(*) => fail fmt!("unit_type should only be called with simple units, not %?", u),
	}
}

pure fn unit_abrev(u: Unit) -> ~str
{
	match u
	{
		// modifiers
		Yocto		=> ~"y",
		Zepto		=> ~"z",
		Atto		=> ~"a",
		Femto		=> ~"f",
		Pico		=> ~"p",
		Nano		=> ~"n",
		Micro		=> ~"u",
		Milli		=> ~"m",
		Centi		=> ~"c",
		Deci		=> ~"d",
		Hecto		=> ~"h",
		Kilo		=> ~"k",
		Mega		=> ~"M",
		Giga		=> ~"G",
		Tera		=> ~"T",
		Peta		=> ~"P",
		Exa		=> ~"E",
		Zetta		=> ~"Z",
		Yotta		=> ~"Y",
		Kibi		=> ~"Ki",
		Mebi		=> ~"Mi",
		Gibi		=> ~"Gi",
		Tebi		=> ~"Ti",
		Pebi		=> ~"Pi",
		Exbi		=> ~"Ei",
		
		// temperature
		Kelvin		=> ~"K",
		Celsius		=> ~"C",
		Fahrenheit		=> ~"F",
		
		// area
		Hectare		=> ~"ha",
		
		// volume
		Liter		=> ~"L",
		Gallon		=> ~"gal",
		
		// length
		Meter		=> ~"m",
		AU		=> ~"ua",
		Inch		=> ~"in",
		Feet		=> ~"ft",
		Yard		=> ~"yd",
		Mile		=> ~"mi",
		NauticalMile		=> ~"nmi",
		
		// mass
		Gram		=> ~"g",
		Tonne		=> ~"t",
		Dram		=> ~"dr",
		Ounce		=> ~"oz",
		Pound		=> ~"lb",
		
		// time
		Second		=> ~"s",
		Minute		=> ~"min",
		Hour		=> ~"h",
		Day		=> ~"d",
		Month		=> ~"mo",
		Year		=> ~"yr",
		
		// amount_of_substance
		Mole		=> ~"mol",
		
		// luminous_intensity
		Candela		=> ~"cd",
		
		// bits
		Bit		=> ~"b",
		Byte		=> ~"B",
		
		// electric_current
		Ampere		=> ~"A",
		
		// compound
		Compound(*)	=> fail fmt!("unit_abrev should only be called with simple units, not %?", u),
	}
}
