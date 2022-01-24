use std::cmp::Ordering;
use std::convert::Infallible;
use std::iter::{Chain, Cloned, Copied, Cycle, Enumerate, Filter, FilterMap, FlatMap, Flatten, Fuse, Inspect, Map, MapWhile, Peekable, Product, Rev, Scan, Skip, SkipWhile, StepBy, Sum, Take, TakeWhile, Zip};

fn main() {
	println!("Hello, world!");

	// str, String
	let s1: String = String::from("Hello, World!");
	let s2: &str = &s1;
	let s3: String = s2.to_string();
	println!("{s3}");

	// tuple
	let mut t = (1, "2");
	t.0 = 2;
	t.1 = "3";

	// Array
	let mut a: [i32; 3] = [0, 1, 2];
	let b: [i32; 3] = [0; 3];
	a[1] = b[1];
	a[2] = b[2];
	println!("{:?}", &a[0..3]);

	let p = Person {
		name: String::from("John"),
		age: 8,
	};

	// Enum
	let e1 = Event::Quit;
	let e2 = Event::MouseDown { x: 10, y: 10 };

	// Vector
	let v = vec![1,2,3,4,5];
	for element in &v {
		println!("{}", element);
	}

	// Box
	let byte_array = [b'h', b'e', b'l', b'l', b'o'];
	print(Box::new(byte_array));

	// match
	let c = Color::Red;
	match c {
		Color::Red => println!("Red"),
		Color::Blue => println!("Blue"),
		Color::Green => println!("Green"),
	}

	let it = Iter {
		current: 0,
		max: 10,
	};
	for num in it {
		println!("{}", num);
	}
}

struct Person {
	name: String,
	age: u32,
}

enum Event {
	Quit,
	KeyDown(u8),
	MouseDown { x: i32, y: i32 },
}

fn print(s: Box<[u8]>) {
	println!("{:?}", s);
}

enum Color {
	Red,
	Blue,
	Green
}

struct Iter {
	current: usize,
	max: usize,
}

impl Iterator for Iter {
	type Item = usize;

	fn next(&mut self) -> Option<Self::Item> {
		self.current += 1;
		if self.current - 1 < self.max {
			Some(self.current - 1)
		} else {
			None
		}
	}
}
