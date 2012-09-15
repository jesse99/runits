#!/usr/bin/python
import os, re, sys, traceback

try:
	import argparse
except:
	sys.stderr.write("This script requires Python 2.7 or later\n")
	sys.exit(2)

options = None

class Unit(object):
	def __init__(self, properties):
		assert len(properties) == 4
		self.__name = properties[0]
		self.__scaling = properties[1]
		self.__canonical = properties[2]
		self.__abrev = properties[3]
	
	@property
	def name(self):
		return self.__name
	
	@property
	def scaling(self):
		return self.__scaling
	
	@property
	def canonical(self):
		return self.__canonical
	
	@property
	def abrev(self):
		return self.__abrev
	
	def __str__(self):
		return "'%s %s %s %s'" % (self.__name, self.__scaling, self.__canonical, self.__abrev)
	
	def __repr__(self):
		return "'%s %s %s %s'" % (self.__name, self.__scaling, self.__canonical, self.__abrev)
	
# Input files are line based where the start of each line controls how it is parsed.
# Start Char	Disposition
# #				ignored (comment)
# whitespace	ignored (error if the line isn't blank)
# :				signals the kind of the units which follow
# upper case	white space separated lost of unit properties				
class ProcessInput(object):
	def process(self):
		try:
			lines = self.__read_file()
			return self.__process_lines(lines)
		except Exception, e:
			sys.stderr.write("Failed to process %s: %s.\n" % (options.src, e))
			if options.verbose >= 1:
				traceback.print_exc(file=sys.stderr)
			sys.exit(1)
				
	# Returns a map where the keys are unit kinds (from directives) and the
	# values are lists of Units.
	def __process_lines(self, lines):
		result = {}
		
		key = None
		for line in lines:
			if len(line) == 1:
				key = line[0]
			else:
				if not key:
					raise Exception("no kind directive for unit '%s'"% ' '.join(line))
				unit = Unit(line)
				result.setdefault(key, []).append(unit)
		
		return result
		
	# Returns an array of arrays. Where the inner arrays look like ['length'] for
	# directives and ['Meter', '1.0', 'Meter', 'm'] for units.
	def __read_file(self):
		lines = []
		i = 0
		with open(options.src, 'r') as f:
			while True:
				line = f.readline()
				i += 1
				
				if line:
					# comment
					if line.startswith('#'):
						pass
					
					# whitespace
					elif line.isspace():
						if line.strip():
							raise Exception("expected a blank line on line %s"% i)
						pass
						
					# directive
					elif line.startswith(':'):
						properties = line[1:].split()
						if len(properties) != 1:
							raise Exception("directive on line %s should have a single kind"% i)
						lines.append(properties)
						
					# unit
					elif line[0].isupper():
						properties = line.split()
						if len(properties) != 4:
							raise Exception("unit on line %s should have four columns"% i)
						lines.append(properties)
						
					# malformed
					else:
						raise Exception("malformed line %s (lines must start with a colon, upper-case letter, a hash, or be blank)"% i)
				else:
					break
		return lines
		
class WriteOutput(object):
	def __init__(self, data):
		self.__data = data
		
	def write(self):
		try:
			with open(options.dst, 'w') as f:
				self.__write_header(f)
				self.__write_exports(f)
				self.__write_enum(f)
		except Exception, e:
			sys.stderr.write("Failed to write %s: %s.\n" % (options.dst, e))
			if options.verbose >= 1:
				traceback.print_exc(file=sys.stderr)
			sys.exit(1)
	
	def __write_header(self, f):
		f.write('// DO NOT EDIT: generated from %s using gen_units.py\n' % options.src)
	
	def __write_exports(self, f):
		f.write('export Unit')
		
		for units in self.__data.values():
			for unit in units:
				f.write(', %s' % unit.name)
		f.write(', Compound')
		f.write(';\n\n')
	
	def __write_enum(self, f):
		f.write('/// Simple units are specified with one of the constructors (e.g. Meter).\n')
		f.write('/// Compound units are constructed using multiplication and division\n')
		f.write('/// (e.g. Meter/(Second*Second)). Dimensionless units are empty Compound\n')
		f.write('/// units.\n')
		f.write('enum Unit\n')
		f.write('{\n')
		for kind, units in self.__data.items():
			f.write('	// %s\n' % kind)
			for unit in units:
				f.write('	%s,\n' % unit.name)
			f.write('	\n')
		
		f.write('	// compound\n')
		f.write('	Compound(@[Unit], @[Unit]),	// numerator, denominator (which must be simple units)\n')
		f.write('}\n')

parser = argparse.ArgumentParser(description = "Generates rust code for units.")
parser.add_argument("--verbose", "-v", action='count', help = 'print extra information')
parser.add_argument("--in", metavar = "FILE", required = True, dest = "src", help = "path to a file describing the units")
parser.add_argument("--out", metavar = "FILE", required = True, dest = "dst", help = "path to use when writing the generated rust code")
options = parser.parse_args()

if not os.path.isfile(options.src):
	print '%s is not a file' % options.src
	sys.exit(1)
if os.path.isdir(options.dst):
	print '%s should point to a file' % options.dst
	sys.exit(1)

process = ProcessInput()
data = process.process()
if options.verbose >= 2:
	print data

writer = WriteOutput(data)
writer.write()
