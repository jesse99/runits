use io::WriterUtil;
use std::map::*;
use core::ops::*;
use generated::*;

export Value;

// ---- Unit ----------------------------------------------------------------------------
impl Unit
{
	pure fn is_dimensionless() -> bool
	{
		match self
		{
			Compound(n, d)	=> n.is_empty() && d.is_empty(),
			_					=> false,
		}
	}
	
	pure fn is_not_dimensionless() -> bool
	{
		!self.is_dimensionless()
	}
}

impl Unit : ops::Mul<Unit, Unit>
{
	pure fn mul(rhs: Unit) -> Unit
	{
		let (numer1, denom1) = to_compound(self);
		let (numer2, denom2) = to_compound(rhs);
		cancel_units(numer1 + numer2, denom1 + denom2)
	}
}

impl Unit : ops::Div<Unit, Unit>
{
	pure fn div(rhs: Unit) -> Unit
	{
		let (numer1, denom1) = to_compound(self);
		let (numer2, denom2) = to_compound(rhs);
		cancel_units(numer1 + denom2, denom1 + numer2)	// division is multiply by reciprocal
	}
}

impl  Unit : ToStr 
{
	fn to_str() -> ~str
	{
		// Bit of an icky function: converts stuff like ["m", "s", "m"] to "m^2*s".
		fn units_to_str(original: @[Unit], units: &[~str], invert: bool) -> ~str
		{
			fn power_count(units: &[~str], start: uint) -> int
			{
				let mut count = 0;
				
				for units.eachi
				|i, u|
				{
					if u == units[start]
					{
						if i < start
						{
							// found an earlier copy of the unit so this one doesn't count
							return 0;
						}
						count += 1;
					}
				}
				
				count
			}
			
			// This is like str::connect except that it checks for empty terms
			// and only adds sep if it is not a modifier.
			fn connect_units(original: @[Unit], units: &[~str]) -> ~str
			{
				let mut result = ~"", first = true;
				for units.eachi
				|i, ss|
				{
					if ss.is_not_empty()
					{
						if first
						{
							first = false;
						}
						else if !is_modifier(original[i-1])
						{
							unchecked {str::push_str(result, ~"*");}
						}
						unchecked {str::push_str(result, ss)};
					}
				}
				result
			}
			
			let x = do units.mapi
			|i, u|
			{
				match power_count(units, i)
				{
					0	=> ~"",
					1	=> if invert {fmt!("%s^-1", u)} else {copy u},
					n	=> fmt!("%s^%?", u, if invert {-n} else {n}),
				}
			};
			
			connect_units(original, x)
		}
		
		match self
		{
			Compound(n, d) if  n.is_empty() && d.is_empty() =>
			{
				~""
			}
			Compound(n, d) =>
			{
				let numer = units_to_str(n, do n.map |u| {u.to_str()}, false);
				let denom = units_to_str(d, do d.map |u| {u.to_str()}, n.is_empty());
				if numer.is_not_empty() && denom.is_not_empty()
				{
					fmt!("%s/%s", numer, denom)
				}
				else if denom.is_not_empty()
				{
					denom
				}
				else
				{
					numer
				}
			}
			u =>
			{
				unit_abrev(u)
			}
		}
	}
}

// TODO: This is hopefully temporary: at some point rust should again be able to compare enums without assistence.
impl Unit : cmp::Eq
{
	pure fn eq(&&rhs: Unit) -> bool
	{
		fmt!("%?", self) == fmt!("%?", rhs)
	}
	
	pure fn ne(&&rhs: Unit) -> bool
	{
		fmt!("%?", self) != fmt!("%?", rhs)
	}
}

// ---- Value ---------------------------------------------------------------------------
/// Values are numbers represented in an arbitrary unit. They support
/// the standard arithmetic operations and fail is called if the units are
/// incompatible (e.g. if meters are added to seconds).
///
/// Note that units are converted to different units only when explicitly
/// directed to do so (e.g. via convert_to). 
struct Value
{
	pub value: float,
	pub units: Unit,		// public so users have more control over stuff like to_str
}

/// Creates a dimensionless value.
pure fn from_number(value: float) -> Value
{
	Value {value: value, units: Compound(@[], @[])}
}

pure fn from_units(value: float, units: Unit) -> Value
{
	match units
	{
		Compound(u, v) if u.len() == 1 && v.is_empty() =>
		{
			Value {value: value, units: u[0]}
		}
		u =>
		{
			Value {value: value, units: u}
		}
	}
}

impl Value
{
	fn convert_to(to: Unit) -> Value
	{
		check_commensurable_units(self.units, to, ~"convert_to");
		let c = to_canonical(self);
		from_canonical(c.value, to)
	}
}

impl Value : ops::Mul<Value, Value>
{
	pure fn mul(rhs: Value) -> Value
	{
		Value {value: self.value * rhs.value, units: self.units*rhs.units}
	}
}

impl Value : ops::Div<Value, Value>
{
	pure fn div(rhs: Value) -> Value
	{
		Value {value: self.value / rhs.value, units: self.units/rhs.units}
	}
}

// Modulus is lhs - (rhs * int(lhs/rhs)) so units is left unchanged.
impl Value : ops::Modulo<Value, Value>
{
	pure fn modulo(rhs: Value) -> Value
	{
		check_identical_units(self.units, rhs.units, ~"modulo");
		Value {value: self.value % rhs.value, units: self.units}
	}
}

impl Value : ops::Add<Value, Value>
{
	pure fn add(rhs: Value) -> Value
	{
		check_identical_units(self.units, rhs.units, ~"add");
		Value {value: self.value + rhs.value, units: self.units}
	}
}

impl Value : ops::Sub<Value, Value>
{
	pure fn sub(rhs: Value) -> Value
	{
		check_identical_units(self.units, rhs.units, ~"sub");
		Value {value: self.value - rhs.value, units: self.units}
	}
}

impl Value : ops::Neg<Value>
{
	pure fn neg() -> Value
	{
		Value {value: -self.value, units: self.units}
	}
}

impl Value : cmp::Ord
{
	pure fn lt(&&rhs: Value) -> bool
	{
		check_identical_units(self.units, rhs.units, ~"lt");
		self.value < rhs.value
	}
	
	pure fn le(&&rhs: Value) -> bool
	{
		check_identical_units(self.units, rhs.units, ~"le");
		self.value <= rhs.value
	}
	
	pure fn ge(&&rhs: Value) -> bool
	{
		check_identical_units(self.units, rhs.units, ~"ge");
		self.value >= rhs.value
	}
	
	pure fn gt(&&rhs: Value) -> bool
	{
		check_identical_units(self.units, rhs.units, ~"gt");
		self.value > rhs.value
	}
}

impl Value : cmp::Eq
{
	pure fn eq(&&rhs: Value) -> bool
	{
		check_identical_units(self.units, rhs.units, ~"eq");
		self.value == rhs.value
	}
	
	pure fn ne(&&rhs: Value) -> bool
	{
		check_identical_units(self.units, rhs.units, ~"ne");
		self.value != rhs.value
	}
}

impl  Value : ToStr 
{
	fn to_str() -> ~str
	{
		if self.units.is_not_dimensionless()
		{
			fmt!("%.f %s", self.value, self.units.to_str())
		}
		else
		{
			fmt!("%.f", self.value)
		}
	}
}

// ---- Internal Items ------------------------------------------------------------------
pure fn to_compound(unit: Unit) -> (@[Unit], @[Unit])
{
	match unit
	{
		Compound(n, d)	=> (n, d),
		u					=> (@[u], @[]),
	}
}

pure fn cancel_units(numer: @[Unit], denom: @[Unit]) -> Unit
{
	pure fn box_remove_at<T: Copy>(v: @[T], index: uint) -> @[T]
	{
		do at_vec::build_sized(v.len() - 1)
		|push|
		{
			for v.eachi
			|i, e|
			{
				if i != index
				{
					push(e);
				}
			}
		}
	}
	
	let mut rnumer = @[];
	let mut rdenom = copy denom;
	
	for numer.each
	|u|
	{
		match denom.position_elem(u)
		{
			option::Some(i)	=> rdenom = box_remove_at(rdenom, i),
			option::None		=> rnumer += @[u],
		}
	}
	
	if rnumer.len() == 1 && rdenom.is_empty()
	{
		rnumer[0]
	}
	else
	{
		Compound(rnumer, rdenom)
	}
}

pure fn to_canonical(x: Value) -> Value
{
	let mut rvalue = x.value;
	let mut rnumer = @[];
	let mut rdenom = @[];
	
	let (numer, denom) =
		match x.units
		{
			Compound(n, d)	=> (n, d),
			u					=> (@[u], @[]),
		};
	
	for numer.each
	|u|
	{
		let (offset, scaling, v) = canonical_unit(u);
		rvalue = (rvalue + offset)*scaling;
		rnumer += v;
	}
	
	for denom.each
	|u|
	{
		let (offset, scaling, v) = canonical_unit(u);
		rvalue = rvalue*(1.0/scaling) - offset;
		rdenom += v;
	}
	
	from_units(rvalue, Compound(rnumer, rdenom))
}

pure fn from_canonical(x: float, u: Unit) -> Value
{
	let mut rvalue = x;
	let mut rnumer = @[];
	let mut rdenom = @[];
	
	let (numer, denom) =
		match u
		{
			Compound(n, d)	=> (n, d),
			u					=> (@[u], @[]),
		};
	
	for numer.each
	|u|
	{
		let (offset, scaling, _v) = canonical_unit(u);
		rvalue = rvalue*(1.0/scaling) - offset;
		rnumer += @[u];
	}
	
	for denom.each
	|u|
	{
		let (offset, scaling, _v) = canonical_unit(u);
		rvalue = (rvalue + offset)*scaling;
		rdenom += @[u];
	}
	
	from_units(rvalue, Compound(rnumer, rdenom))
}

// Fails if the units are different.
pure fn check_identical_units(lhs: Unit, rhs: Unit, fname: &str)
{
	pure fn unit_name(u: Unit) -> ~str {fmt!("%?", u)}
	check_compatible_units(lhs, rhs, fname, unit_name)
}

// Fails if the unit kinds are different.
pure fn check_commensurable_units(lhs: Unit, rhs: Unit, fname: &str)
{
	check_compatible_units(lhs, rhs, fname, unit_type)
}

pure fn check_compatible_units(lhs: Unit, rhs: Unit, fname: &str, kind: fn@ (u: Unit) -> ~str)
{
	fn increment_type(numer: hashmap<@~str, uint>, denom: hashmap<@~str, uint>, u: Unit, kind: fn@ (u: Unit) -> ~str)
	{
		fn increment(table: hashmap<@~str, uint>, u: Unit, kind: fn@ (u: Unit) -> ~str)
		{
			let key = @kind(u);
			if key.is_not_empty()
			{
				match table.find(key)
				{
					option::Some(count)	=> table.insert(key, count + 1),
					option::None			=> table.insert(key, 1),
				};
			}
		}
		
		match u
		{
			Compound(n, d)	=>
			{
				for n.each |v| {increment(numer, v, kind)}
				for d.each |v| {increment(denom, v, kind)}
			}
			_ => {increment(numer, u, kind)}
		}
	}
	
	unchecked
	{
		let numer1 = box_str_hash();
		let denom1 = box_str_hash();
		increment_type(numer1, denom1, lhs, kind);
		
		let numer2 = box_str_hash();
		let denom2 = box_str_hash();
		increment_type(numer2, denom2, rhs, kind);
		
		if numer1 != numer2 || denom1 != denom2
		{
			fail fmt!("incompatible units for `%s`.%s(`%s`)", lhs.to_str(), fname, rhs.to_str());
		}
	}
}

// ---- Tests ---------------------------------------------------------------------------
#[cfg(test)]
fn check_strings(actual: &a/str, expected: &a/str) -> bool
{
	if actual != expected
	{
		io::stderr().write_line(fmt!("Found %? but expected %?", actual, expected));
		return false;
	}
	return true;
}

#[cfg(test)]
fn check_floats(actual: float, expected: float) -> bool
{
	if float::abs(actual - expected) > 0.001
	{
		io::stderr().write_line(fmt!("Found %f but expected %f", actual, expected));
		return false;
	}
	return true;
}

#[cfg(test)]
fn check_units(actual: Unit, expected: Unit) -> bool
{
	if fmt!("%?", actual) != fmt!("%?", expected)	// TODO: use != when enums again support equality
	{
		io::stderr().write_line(fmt!("Found %? but expected %?", actual, expected));
		return false;
	}
	return true;
}

#[test]
fn test_mul_unit()
{
	assert check_units(Meter*Meter, Compound(@[Meter, Meter], @[]));
	assert check_units(Kilo*Second, Compound(@[Kilo, Second], @[]));
	assert check_units((Meter*Meter)*(Second*Second), Compound(@[Meter, Meter, Second, Second], @[]));
}

#[test]
fn test_div_unit()
{
	assert check_units(Meter/Second, Compound(@[Meter], @[Second]));
	assert check_units(Second*Meter/Second, Meter);
	assert check_units(Second*(Meter/Second), Meter);
	assert check_units(Second*Meter/(Meter*Second*Second), Compound(@[], @[Second]));
}

#[test]
fn test_mul_value()
{
	let x = from_number(5.0)*from_units(3.0, Meter);
	assert check_floats(x.value, 15.0);
	assert check_units(x.units, Meter);
	
	let x = from_units(5.0, Meter)*from_units(3.0, Meter);
	assert check_floats(x.value, 15.0);
	assert check_units(x.units, Compound(@[Meter, Meter], @[]));
}

#[test]
fn test_div_value()
{
	let x = from_number(5.0)/from_units(2.0, Meter);
	assert check_floats(x.value, 2.5);
	assert check_units(x.units, Compound(@[], @[Meter]));
	
	let x = from_units(5.0, Meter)/from_units(2.0, Meter);
	assert check_floats(x.value, 2.5);
	assert check_units(x.units, Compound(@[], @[]));
	
	let x = from_units(5.0, Meter*Second)/from_units(2.0, Second);
	assert check_floats(x.value, 2.5);
	assert check_units(x.units, Meter);
	
	let x = from_units(5.0, Meter)/from_units(2.0, Second);
	assert check_floats(x.value, 2.5);
	assert check_units(x.units, Compound(@[Meter], @[Second]));
	
	let x = from_units(5.0, Meter)/from_units(2.0, Meter*Second);
	assert check_floats(x.value, 2.5);
	assert check_units(x.units, Compound(@[], @[Second]));
}

#[test]
fn test_to_canonical()
{
	let x = to_canonical(from_units(3.0, Meter));
	assert check_floats(x.value, 3.0);
	assert check_units(x.units, Meter);
	
	let x = to_canonical(from_units(3.0, Feet));
	assert check_floats(x.value, 0.9144);
	assert check_units(x.units, Meter);
	
	let x = to_canonical(from_units(3.0, Kilo*Meter));
	assert check_floats(x.value, 3000.0);
	assert check_units(x.units, Meter);
	
	let x = to_canonical(from_units(3.0, Kilo*Feet));
	assert check_floats(x.value, 914.4);
	assert check_units(x.units, Meter);
	
	let x = to_canonical(from_units(3.0, Feet)/from_units(2.0, Minute));
	assert check_floats(x.value, 0.00762);
	assert check_units(x.units, Compound(@[Meter], @[Second]));
}

#[test]
fn test_from_canonical()
{
	let x = from_canonical(3.0, Meter);
	assert check_floats(x.value, 3.0);
	assert check_units(x.units, Meter);
	
	let x = from_canonical(3.0, Feet);
	assert check_floats(x.value, 9.8425197);
	assert check_units(x.units, Feet);
	
	let x = from_canonical(3000.0, Kilo*Meter);
	assert check_floats(x.value, 3.0);
	assert check_units(x.units, Compound(@[Kilo, Meter], @[]));
	
	let x = from_canonical(914.4, Kilo*Feet);
	assert check_floats(x.value, 3.0);
	assert check_units(x.units, Compound(@[Kilo, Feet], @[]));
}

#[test]
fn test_convert_to()
{
	let x = from_units(5.0, Feet).convert_to(Meter);
	assert check_floats(x.value, 1.524);
	assert check_units(x.units, Meter);
	
	let x = from_units(5.0, Kilo*Meter).convert_to(Feet);
	assert check_floats(x.value, 16_404.199);
	assert check_units(x.units, Feet);
	
	let x = from_units(5.0, Kilo*Meter).convert_to(Milli*Feet);
	assert check_floats(x.value, 16_404_199.475);
	assert check_units(x.units, Milli*Feet);
	
	let x = from_units(5.0, Meter/Second).convert_to(Feet/Minute);
	assert check_floats(x.value, 984.25197);
	assert check_units(x.units, Feet/Minute);
	
	let x = from_units(5.0, Kilo*Meter*Second).convert_to(Second*Milli*Feet);
	assert check_floats(x.value, 16_404_199.475);
	assert check_units(x.units, Second*Milli*Feet);
}

#[test]
#[should_fail]
fn test_incommensurable_convert()
{
	let x = from_units(5.0, Feet).convert_to(Second);
	assert x.value > 0.0;
}

#[test]
fn test_value_to_str()
{
	let x = from_number(5.0);
	assert check_strings(x.to_str(), ~"5");
	
	let x = from_units(5.0, Meter);
	assert check_strings(x.to_str(), ~"5 m");
	
	let x = from_units(5.0, Meter*Meter);
	assert check_strings(x.to_str(), ~"5 m^2");
	
	let x = from_units(5.0, Meter*Second*Meter);
	assert check_strings(x.to_str(), ~"5 m^2*s");
	
	let x = from_units(5.0, Meter/Second);
	assert check_strings(x.to_str(), ~"5 m/s");
	
	let x = from_units(5.0, Meter/(Kilo*Second*Second));
	assert check_strings(x.to_str(), ~"5 m/ks^2");
	
	let x = from_number(10.0)/from_units(5.0, Meter);
	assert check_strings(x.to_str(), ~"2 m^-1");
	
	let x = from_number(10.0)/from_units(5.0, Meter*Meter);
	assert check_strings(x.to_str(), ~"2 m^-2");
}

#[test]
#[should_fail]
fn test_incompatible_lt()
{
	let x = from_units(5.0, Feet);
	let y = from_units(2.0, Second);
	assert x < y;
}

#[test]
fn test_value_add()
{
	let x = from_units(5.0, Meter);
	let y = from_units(3.0, Meter);
	let z = x + y;
	assert check_floats(z.value, 8.0);
	assert check_units(z.units, Meter);
}

#[test]
#[should_fail]
fn test_incompatible_add()
{
	let x = from_units(5.0, Feet);
	let y = from_units(2.0, Meter);
	let z = x + y;
	assert z.value > 0.0;
}

#[test]
fn test_temperature_convert_to()
{
	let x = from_units(42.0, Celsius).convert_to(Fahrenheit);
	assert check_floats(x.value, 107.595462);
	assert check_units(x.units, Fahrenheit);
	
	let x = from_units(50.0, Fahrenheit).convert_to(Celsius);
	assert check_floats(x.value, 10.002265);
	assert check_units(x.units, Celsius);
}

#[test]
fn test_derived()
{
	let x = from_units(3.0, Liter).convert_to(Gallon);
	assert check_floats(x.value, 0.79251616);
	assert check_units(x.units, Gallon);
}

#[test]
#[should_fail]
fn test_incompaible_derived()
{
	let x = from_units(3.0, Hectare).convert_to(Gallon);
	assert x.value > 0.0;
}
