// Last compiled with rust 6b670c306b8de545afcbcea81bcd592c644409d7
use io::WriterUtil;
use std::map::*;
use core::ops::*;
use generated::*;

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

/// For binary ops the rhs is converted to the units of the lhs.
impl Unit : ops::Mul<Unit, Unit>
{
	pure fn mul(&&rhs: Unit) -> Unit
	{
		let (numer1, denom1) = to_compound(self);
		let (numer2, denom2) = to_compound(rhs);
		cancel_units(numer1 + numer2, denom1 + denom2)
	}
}

impl Unit : ops::Div<Unit, Unit>
{
	pure fn div(&&rhs: Unit) -> Unit
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
		do_units_to_str(self)
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
pub struct Value
{
	pub value: float,
	pub units: Unit,		// public so users have more control over stuff like to_str
}

/// Creates a dimensionless value.
pub pure fn from_number(value: float) -> Value
{
	Value {value: value, units: Compound(@[], @[])}
}

pub pure fn from_units(value: float, units: Unit) -> Value
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
	pure fn convert_to(to: Unit) -> Value
	{
		convert_to(self, to, ~"convert_to")
	}
	
	/// Adjusts prefix such that value >= 1 && value < 1000.
	///
	/// Note that this will only select a prefix that is a power of three (so Centi, Deci,
	/// and Hecto are not used).
	pure fn normalize_si() -> Value
	{
		let mut value = remove_modifiers(self);
		
		for si_modifiers
		|u|
		{
			if u != Centi && u != Deci && u != Hecto
			{
				let candidate = apply_modifier(value, u);
				let x = float::abs(candidate.value);
				if x >= 1.0 && x < 1000.0
				{
					value = candidate;
					break;
				}
			}
		}
		
		value
	}
	
	/// Adjusts prefix such that value >= 1 && value < 1024.
	pure fn normalize_binary() -> Value
	{
		let mut value = remove_modifiers(self);
		
		for binary_modifiers
		|u|
		{
			let candidate = apply_modifier(value, u);
			let x = float::abs(candidate.value);
			if x >= 1.0 && x < 1024.0
			{
				value = candidate;
				break;
			}
		}
		
		value
	}
	
	/// If self is less than 1s then use normalize_si. Otherwise set units to the
	/// largest time unit such that value >= 1.0.
	///
	/// Fails if value is not a simple time.
	pure fn normalize_time() -> Value
	{
		let value = self.convert_to(Second);
		if value.value < 1.0
		{
			value.normalize_si()
		}
		else if (value.value >= 60.0)
		{
			let x = value.convert_to(Year);
			if float::abs(x.value) >= 1.0 {return x}
			
			let x = value.convert_to(Month);
			if float::abs(x.value) >= 1.0 {return x}
			
			let x = value.convert_to(Day);
			if float::abs(x.value) >= 1.0 {return x}
			
			let x = value.convert_to(Hour);
			if float::abs(x.value) >= 1.0 {return x}
			
			value.convert_to(Minute)
		}
		else
		{
			value
		}
	}
}

impl Value : ops::Mul<Value, Value>
{
	pure fn mul(&&rhs: Value) -> Value
	{
		Value {value: self.value * rhs.value, units: self.units*rhs.units}
	}
}

impl Value : ops::Div<Value, Value>
{
	pure fn div(&&rhs: Value) -> Value
	{
		Value {value: self.value / rhs.value, units: self.units/rhs.units}
	}
}

// Modulus is lhs - (rhs * int(lhs/rhs)) so units is left unchanged.
impl Value : ops::Modulo<Value, Value>
{
	pure fn modulo(&&rhs: Value) -> Value
	{
		let rhs = convert_to(rhs, self.units, ~"modulo");
		Value {value: self.value % rhs.value, units: self.units}
	}
}

impl Value : ops::Add<Value, Value>
{
	pure fn add(&&rhs: Value) -> Value
	{
		let rhs = convert_to(rhs, self.units, ~"add");
		Value {value: self.value + rhs.value, units: self.units}
	}
}

impl Value : ops::Sub<Value, Value>
{
	pure fn sub(&&rhs: Value) -> Value
	{
		let rhs = convert_to(rhs, self.units, ~"sub");
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
		let rhs = convert_to(rhs, self.units, ~"lt");
		self.value < rhs.value
	}
	
	pure fn le(&&rhs: Value) -> bool
	{
		let rhs = convert_to(rhs, self.units, ~"le");
		self.value <= rhs.value
	}
	
	pure fn ge(&&rhs: Value) -> bool
	{
		let rhs = convert_to(rhs, self.units, ~"ge");
		self.value >= rhs.value
	}
	
	pure fn gt(&&rhs: Value) -> bool
	{
		let rhs = convert_to(rhs, self.units, ~"gt");
		self.value > rhs.value
	}
}

impl Value : cmp::Eq
{
	pure fn eq(&&rhs: Value) -> bool
	{
		let rhs = convert_to(rhs, self.units, ~"eq");
		self.value == rhs.value
	}
	
	pure fn ne(&&rhs: Value) -> bool
	{
		let rhs = convert_to(rhs, self.units, ~"ne");
		self.value != rhs.value
	}
}

impl  Value : ToStr 
{
	fn to_str() -> ~str
	{
		if self.units.is_not_dimensionless()
		{
			fmt!("%f %s", self.value, self.units.to_str())
		}
		else
		{
			fmt!("%f", self.value)
		}
	}
}

// ---- Internal Items ------------------------------------------------------------------
fn do_units_to_str(unit: Unit) -> ~str
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
						unsafe {str::push_str(result, ~"*");}
					}
					unsafe {str::push_str(result, ss)};
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
	
	match unit
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

pure fn apply_modifier(x: Value, u: Unit) -> Value
{
	let (_offset, scaling, _numer, _denom) = canonical_unit(u);
	Value
	{
		value: x.value/scaling,
		units:
			match x.units
			{
				Compound(n, d)	=> Compound(@[u] + n, d),
				v					=> Compound(@[u, v], @[]),
			}
	}
}

pure fn convert_to(value: Value, to: Unit, fname: ~str) -> Value
{
	if value.units == to
	{
		value
	}
	else
	{
		check_commensurable(value, to, fname);
		let c = to_canonical(value);
		from_canonical(c.value, to)
	}
}

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
		match denom.position_elem(*u)
		{
			option::Some(i)	=> rdenom = box_remove_at(rdenom, i),
			option::None		=> rnumer += @[*u],
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
		let (offset, scaling, n, d) = canonical_unit(*u);
		rvalue = (rvalue + offset)*scaling;
		rnumer += n;
		rdenom += d;
	}
	
	for denom.each
	|u|
	{
		let (offset, scaling, n, d) = canonical_unit(*u);
		rvalue = rvalue*(1.0/scaling) - offset;
		rnumer += d;
		rdenom += n;
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
		let (offset, scaling, _n, _d) = canonical_unit(*u);
		rvalue = rvalue*(1.0/scaling) - offset;
		rnumer += @[*u];
	}
	
	for denom.each
	|u|
	{
		let (offset, scaling, _n, _d) = canonical_unit(*u);
		rvalue = (rvalue + offset)*scaling;
		rdenom += @[*u];
	}
	
	from_units(rvalue, Compound(rnumer, rdenom))
}

fn hash_equals(lhs: HashMap<@~str, uint>, rhs: HashMap<@~str, uint>) -> bool
{
	if lhs.size() == rhs.size()
	{
		for lhs.each
		|key, value1|
		{
			match rhs.find(key)
			{
				option::Some(value2) =>
				{
					if value1 != value2
					{
						return false;
					}
				}
				option::None =>
				{
					return false;
				}
			}
		}
	}
	
	true
}

// Fails if the unit kinds are different.
pure fn check_commensurable(lhs: Value, rhs: Unit, fname: &str)
{
	fn increment_type(numer: HashMap<@~str, uint>, denom: HashMap<@~str, uint>, u: Unit)
	{
		fn increment(table: HashMap<@~str, uint>, u: Unit)
		{
			let key = @unit_type(u);
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
				for n.each |v| {increment(numer, *v)}
				for d.each |v| {increment(denom, *v)}
			}
			_ => {increment(numer, u)}
		}
	}
	
	unsafe
	{
		let numer1 = HashMap();
		let denom1 = HashMap();
		let lhs2 = to_canonical(lhs);
		increment_type(numer1, denom1, lhs2.units);
		
		let numer2 = HashMap();
		let denom2 = HashMap();
		let rhs2 = to_canonical(from_units(1.0, rhs));
		increment_type(numer2, denom2, rhs2.units);
		
		// TODO: don't use to_str
		if !hash_equals(numer1, numer2) || !hash_equals(denom1, denom2)
		{
			if str::eq_slice(fname, ~"convert_to")
			{
				fail fmt!("incompatible units for `%s`.%s(`%s`)", lhs.to_str(), fname, rhs.to_str());
			}
			else
			{
				// For everything but convert_to this is called with lhs and rhs swapped.
				fail fmt!("incompatible units for `%s`.%s(`%s`)", rhs.to_str(), fname, lhs.to_str());
			}
		}
	}
}

pure fn remove_modifiers(x: Value) -> Value
{
	pure fn remove(uu: @[Unit]) -> (float, @[Unit])
	{
		let mut scaling = 1.0;
		let mut units = @[];
		
		for uu.each
		|u|
		{
			if is_modifier(*u)
			{
				let (_offset, s, _numer, _denom) = canonical_unit(*u);
				scaling *= s;
			}
			else
			{
				units += @[*u];
			}
		}
		
		(scaling, units)
	}
	
	let mut value = x.value;
	let mut numer = @[];
	let mut denom = @[];
	
	match x.units
	{
		Compound(n, d) =>
		{
			let (s, nn) = remove(n);
			let (t, dd) = remove(d);
			value = value*s/t;
			numer = nn;
			denom = dd;
		}
		u =>
		{
			numer += @[u];
		}
	}
	
	if numer.len() == 1 && denom.is_empty()
	{
		from_units(value, numer[0])
	}
	else
	{
		from_units(value, Compound(numer, denom))
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
fn test_compatible_add()
{
	let x = from_units(5.0, Feet);
	let y = from_units(2.0, Meter);
	let z = x + y;
	assert check_floats(z.value, 5.0+6.5616798);
	assert check_units(z.units, Feet);
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

#[test]
fn test_math()
{
	let x = from_units(3.0, Watt) + from_units(10.0, Joule)/from_units(2.0, Second);
	assert check_floats(x.value, 8.0);
	assert check_units(x.units, Watt);
}

#[test]
fn test_remove_modifiers()
{
	let x = remove_modifiers(from_units(3.0, Kilo*Watt));
	assert check_floats(x.value, 3000.0);
	assert check_units(x.units, Watt);
	
	let x = remove_modifiers(from_units(3.0, Meter/(Centi*Second)));
	assert check_floats(x.value, 300.0);
	assert check_units(x.units, Meter/Second);
}

#[test]
fn test_normalize_si()
{
	let x = from_units(3.0, Meter).normalize_si();
	assert check_floats(x.value, 3.0);
	assert check_units(x.units, Meter);
	
	let x = from_units(1025.0, Meter).normalize_si();
	assert check_floats(x.value, 1.025);
	assert check_units(x.units, Kilo*Meter);
	
	let x = from_units(0.000_000_025, Kilo*Meter).normalize_si();
	assert check_floats(x.value, 25.0);
	assert check_units(x.units, Micro*Meter);
	
	let x = from_units(-1025.0, Meter).normalize_si();
	assert check_floats(x.value, -1.025);
	assert check_units(x.units, Kilo*Meter);
	
	let x = from_units(0.000_123, Meter).normalize_si();
	assert check_floats(x.value, 123.0);
	assert check_units(x.units, Micro*Meter);
	
	let x = from_units(-0.000_023, Meter).normalize_si();
	assert check_floats(x.value, -23.0);
	assert check_units(x.units, Micro*Meter);
	
	let x = from_units(0.000_003, Meter).normalize_si();
	assert check_floats(x.value, 3.0);
	assert check_units(x.units, Micro*Meter);
}

#[test]
fn test_normalize_binary()
{
	let x = from_units(3.0, Byte).normalize_binary();
	assert check_floats(x.value, 3.0);
	assert check_units(x.units, Byte);
	
	let x = from_units(1025.0, Byte).normalize_binary();
	assert check_floats(x.value, 1025.0/1024.0);
	assert check_units(x.units, Kibi*Byte);
	
	let x = from_units(-1025.0, Byte).normalize_binary();
	assert check_floats(x.value, -1025.0/1024.0);
	assert check_units(x.units, Kibi*Byte);
}

#[test]
fn test_normalize_time()
{
	let x = from_units(5.0, Second).normalize_time();
	assert check_floats(x.value, 5.0);
	assert check_units(x.units, Second);
	
	let x = from_units(0.033, Second).normalize_time();
	assert check_floats(x.value, 33.0);
	assert check_units(x.units, Milli*Second);
	
	let x = from_units(33.0, Day).normalize_time();
	assert check_floats(x.value, 1.084211);
	assert check_units(x.units, Month);
}

// From the README
#[test]
fn test_usage()
{
	let speed = from_units(30.0, Mile/Hour);
	let delta = from_units(2.0, Meter/Second);
	let sum = speed + delta;   // for binary ops the rhs is converted to the lhs units
	info!("sum = %s", sum.to_str()); // prints "34.473873 mi/h"
	
	let sum = sum.convert_to(Kilo*Meter/Second);
	info!("speed1 = %s", sum.to_str()); // prints "0.015411 km/s"
	
	let sum = sum.normalize_si();
	info!("speed2 = %s", sum.to_str()); // prints "15.411200 m/s"
}
