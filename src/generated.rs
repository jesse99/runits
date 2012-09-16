// DO NOT EDIT: generated from units.in using gen_units.py
export Unit, Newton, Joule, Ohm, Hertz, Volt, Kelvin, Celsius, Fahrenheit, Hectare, Tesla, Bit, Byte, Siemens, Watt, Liter, CubicInch, CubicFeet, CubicYard, Pint, Quart, Gallon, Pascal, Henry, Mole, Candela, Ampere, Yocto, Zepto, Atto, Femto, Pico, Nano, Micro, Milli, Centi, Deci, Hecto, Kilo, Mega, Giga, Tera, Peta, Exa, Zetta, Yotta, Kibi, Mebi, Gibi, Tebi, Pebi, Exbi, Weber, Meter, AU, Inch, Feet, Yard, Mile, NauticalMile, LightYear, Parsec, Coulomb, Gram, Tonne, Dram, Ounce, Pound, Farad, Second, Minute, Hour, Day, Month, Year, Compound;
export canonical_unit, is_modifier, unit_type, unit_abrev;	// these are really internal items

/// Simple units are specified with one of the constructors (e.g. Meter).
/// Compound units are constructed using multiplication and division
/// (e.g. Meter/(Second*Second)). Dimensionless units are empty Compound
/// units.
enum Unit
{
	// force
	Newton,
	
	// energy
	Joule,
	
	// electric_resistance
	Ohm,
	
	// frequency
	Hertz,
	
	// voltage
	Volt,
	
	// temperature
	Kelvin,
	Celsius,
	Fahrenheit,
	
	// area
	Hectare,
	
	// magnetic_field_strength
	Tesla,
	
	// bits
	Bit,
	Byte,
	
	// electric_conductance
	Siemens,
	
	// power
	Watt,
	
	// volume
	Liter,
	CubicInch,
	CubicFeet,
	CubicYard,
	Pint,
	Quart,
	Gallon,
	
	// pressure
	Pascal,
	
	// inductance
	Henry,
	
	// amount_of_substance
	Mole,
	
	// luminous_intensity
	Candela,
	
	// electric_current
	Ampere,
	
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
	
	// magnetic_flux
	Weber,
	
	// length
	Meter,
	AU,
	Inch,
	Feet,
	Yard,
	Mile,
	NauticalMile,
	LightYear,
	Parsec,
	
	// electric_charge
	Coulomb,
	
	// mass
	Gram,
	Tonne,
	Dram,
	Ounce,
	Pound,
	
	// electric_capacitance
	Farad,
	
	// time
	Second,
	Minute,
	Hour,
	Day,
	Month,
	Year,
	
	// compound
	Compound(@[Unit], @[Unit]),	// numerator, denominator (which must be simple units)
}

// Returns (offset, scaling, numer, denom).
pure fn canonical_unit(u: Unit) -> (float, float, @[Unit], @[Unit])
{
	match u
	{
		// force
		Newton			=> (0.0, 1.0, @[Kilo,Gram,Meter], @[Second,Second]),
		
		// energy
		Joule			=> (0.0, 1.0, @[Kilo,Gram,Meter,Meter], @[Second,Second]),
		
		// electric_resistance
		Ohm			=> (0.0, 1.0, @[Kilo,Gram,Meter,Meter], @[Second,Second,Second,Ampere,Ampere]),
		
		// frequency
		Hertz			=> (0.0, 1.0, @[], @[Second]),
		
		// voltage
		Volt			=> (0.0, 1.0, @[Kilo,Gram,Meter,Meter], @[Second,Second,Second,Ampere]),
		
		// temperature
		Kelvin			=> (0.0, 1.0, @[Kelvin], @[]),
		Celsius			=> (273.15, 1.0, @[Kelvin], @[]),
		Fahrenheit			=> (459.67, 0.55556, @[Kelvin], @[]),
		
		// area
		Hectare			=> (0.0, 10000.0, @[Meter,Meter], @[]),
		
		// magnetic_field_strength
		Tesla			=> (0.0, 1.0, @[Kilo,Gram], @[Second,Second,Ampere]),
		
		// bits
		Bit			=> (0.0, 1.0, @[Bit], @[]),
		Byte			=> (0.0, 8.0, @[Bit], @[]),
		
		// electric_conductance
		Siemens			=> (0.0, 1.0, @[Second,Second,Second,Ampere,Ampere], @[Kilo,Gram,Meter,Meter]),
		
		// power
		Watt			=> (0.0, 1.0, @[Kilo,Gram,Meter,Meter], @[Second,Second,Second]),
		
		// volume
		Liter			=> (0.0, 0.001, @[Meter,Meter,Meter], @[]),
		CubicInch			=> (0.0, 0.000016387064, @[Meter,Meter,Meter], @[]),
		CubicFeet			=> (0.0, 0.028316846592, @[Meter,Meter,Meter], @[]),
		CubicYard			=> (0.0, 0.764554857984, @[Meter,Meter,Meter], @[]),
		Pint			=> (0.0, 0.000473176473, @[Meter,Meter,Meter], @[]),
		Quart			=> (0.0, 0.000946352946, @[Meter,Meter,Meter], @[]),
		Gallon			=> (0.0, 0.003785411784, @[Meter,Meter,Meter], @[]),
		
		// pressure
		Pascal			=> (0.0, 1.0, @[Kilo,Gram], @[Meter,Second,Second]),
		
		// inductance
		Henry			=> (0.0, 1.0, @[Kilo,Gram,Meter,Meter], @[Second,Second,Ampere,Ampere]),
		
		// amount_of_substance
		Mole			=> (0.0, 1.0, @[Mole], @[]),
		
		// luminous_intensity
		Candela			=> (0.0, 1.0, @[Candela], @[]),
		
		// electric_current
		Ampere			=> (0.0, 1.0, @[Ampere], @[]),
		
		// magnetic_flux
		Weber			=> (0.0, 1.0, @[Kilo,Gram,Meter,Meter], @[Second,Second,Ampere]),
		
		// length
		Meter			=> (0.0, 1.0, @[Meter], @[]),
		AU			=> (0.0, 1.49597870691e11, @[Meter], @[]),
		Inch			=> (0.0, 0.0254, @[Meter], @[]),
		Feet			=> (0.0, 0.3048, @[Meter], @[]),
		Yard			=> (0.0, 0.9144, @[Meter], @[]),
		Mile			=> (0.0, 1609.344, @[Meter], @[]),
		NauticalMile			=> (0.0, 1852.0, @[Meter], @[]),
		LightYear			=> (0.0, 9460730472580800.0, @[Meter], @[]),
		Parsec			=> (0.0, 30.857e15, @[Meter], @[]),
		
		// electric_charge
		Coulomb			=> (0.0, 1.0, @[Second,Ampere], @[]),
		
		// mass
		Gram			=> (0.0, 1.0, @[Gram], @[]),
		Tonne			=> (0.0, 1.0e6, @[Gram], @[]),
		Dram			=> (0.0, 1.7718451953, @[Gram], @[]),
		Ounce			=> (0.0, 28.349523125, @[Gram], @[]),
		Pound			=> (0.0, 453.59237, @[Gram], @[]),
		
		// electric_capacitance
		Farad			=> (0.0, 1.0, @[Second,Second,Second,Second,Ampere,Ampere], @[Kilo,Gram,Meter,Meter]),
		
		// time
		Second			=> (0.0, 1.0, @[Second], @[]),
		Minute			=> (0.0, 60.0, @[Second], @[]),
		Hour			=> (0.0, 3600.0, @[Second], @[]),
		Day			=> (0.0, 86400.0, @[Second], @[]),
		Month			=> (0.0, 2629746.0, @[Second], @[]),
		Year			=> (0.0, 31557600.0, @[Second], @[]),
		
		// modifiers
		Yocto			=> (0.0, 1.0e-24, @[], @[]),
		Zepto			=> (0.0, 1.0e-21, @[], @[]),
		Atto			=> (0.0, 1.0e-18, @[], @[]),
		Femto			=> (0.0, 1.0e-15, @[], @[]),
		Pico			=> (0.0, 1.0e-12, @[], @[]),
		Nano			=> (0.0, 1.0e-9, @[], @[]),
		Micro			=> (0.0, 1.0e-6, @[], @[]),
		Milli			=> (0.0, 1.0e-3, @[], @[]),
		Centi			=> (0.0, 1.0e-2, @[], @[]),
		Deci			=> (0.0, 1.0e-1, @[], @[]),
		Hecto			=> (0.0, 1.0e2, @[], @[]),
		Kilo			=> (0.0, 1.0e3, @[], @[]),
		Mega			=> (0.0, 1.0e6, @[], @[]),
		Giga			=> (0.0, 1.0e9, @[], @[]),
		Tera			=> (0.0, 1.0e12, @[], @[]),
		Peta			=> (0.0, 1.0e15, @[], @[]),
		Exa			=> (0.0, 1.0e18, @[], @[]),
		Zetta			=> (0.0, 1.0e21, @[], @[]),
		Yotta			=> (0.0, 1.0e24, @[], @[]),
		Kibi			=> (0.0, 1024.0, @[], @[]),
		Mebi			=> (0.0, 1048576.0, @[], @[]),
		Gibi			=> (0.0, 1073741824.0, @[], @[]),
		Tebi			=> (0.0, 1099511627776.0, @[], @[]),
		Pebi			=> (0.0, 1125899906842624.0, @[], @[]),
		Exbi			=> (0.0, 1152921504606846976.0, @[], @[]),
		
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
		Newton => ~"force",
		Joule => ~"energy",
		Ohm => ~"electric_resistance",
		Hertz => ~"frequency",
		Volt => ~"voltage",
		Kelvin | Celsius | Fahrenheit => ~"temperature",
		Hectare => ~"area",
		Tesla => ~"magnetic_field_strength",
		Bit | Byte => ~"bits",
		Siemens => ~"electric_conductance",
		Watt => ~"power",
		Liter | CubicInch | CubicFeet | CubicYard | Pint | Quart | Gallon => ~"volume",
		Pascal => ~"pressure",
		Henry => ~"inductance",
		Mole => ~"amount_of_substance",
		Candela => ~"luminous_intensity",
		Ampere => ~"electric_current",
		Yocto | Zepto | Atto | Femto | Pico | Nano | Micro | Milli | Centi | Deci | Hecto | Kilo | Mega | Giga | Tera | Peta | Exa | Zetta | Yotta | Kibi | Mebi | Gibi | Tebi | Pebi | Exbi => ~"",
		Weber => ~"magnetic_flux",
		Meter | AU | Inch | Feet | Yard | Mile | NauticalMile | LightYear | Parsec => ~"length",
		Coulomb => ~"electric_charge",
		Gram | Tonne | Dram | Ounce | Pound => ~"mass",
		Farad => ~"electric_capacitance",
		Second | Minute | Hour | Day | Month | Year => ~"time",
		Compound(*) => fail fmt!("unit_type should only be called with simple units, not %?", u),
	}
}

pure fn unit_abrev(u: Unit) -> ~str
{
	match u
	{
		// force
		Newton		=> ~"N",
		
		// energy
		Joule		=> ~"J",
		
		// electric_resistance
		Ohm		=> ~"ohm",
		
		// frequency
		Hertz		=> ~"Hz",
		
		// voltage
		Volt		=> ~"V",
		
		// temperature
		Kelvin		=> ~"K",
		Celsius		=> ~"C",
		Fahrenheit		=> ~"F",
		
		// area
		Hectare		=> ~"ha",
		
		// magnetic_field_strength
		Tesla		=> ~"T",
		
		// bits
		Bit		=> ~"b",
		Byte		=> ~"B",
		
		// electric_conductance
		Siemens		=> ~"S",
		
		// power
		Watt		=> ~"W",
		
		// volume
		Liter		=> ~"L",
		CubicInch		=> ~"in^3",
		CubicFeet		=> ~"ft^3",
		CubicYard		=> ~"yd^3",
		Pint		=> ~"pt",
		Quart		=> ~"qt",
		Gallon		=> ~"gal",
		
		// pressure
		Pascal		=> ~"Pa",
		
		// inductance
		Henry		=> ~"H",
		
		// amount_of_substance
		Mole		=> ~"mol",
		
		// luminous_intensity
		Candela		=> ~"cd",
		
		// electric_current
		Ampere		=> ~"A",
		
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
		
		// magnetic_flux
		Weber		=> ~"Wb",
		
		// length
		Meter		=> ~"m",
		AU		=> ~"ua",
		Inch		=> ~"in",
		Feet		=> ~"ft",
		Yard		=> ~"yd",
		Mile		=> ~"mi",
		NauticalMile		=> ~"nmi",
		LightYear		=> ~"ly",
		Parsec		=> ~"pc",
		
		// electric_charge
		Coulomb		=> ~"C",
		
		// mass
		Gram		=> ~"g",
		Tonne		=> ~"t",
		Dram		=> ~"dr",
		Ounce		=> ~"oz",
		Pound		=> ~"lb",
		
		// electric_capacitance
		Farad		=> ~"F",
		
		// time
		Second		=> ~"s",
		Minute		=> ~"min",
		Hour		=> ~"h",
		Day		=> ~"d",
		Month		=> ~"mo",
		Year		=> ~"yr",
		
		// compound
		Compound(*)	=> fail fmt!("unit_abrev should only be called with simple units, not %?", u),
	}
}
