#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    println!("{:?}", mystruct{ _bitfield_1: mystruct::new_bitfield_1(0, // a
								     0, // b
								     0, // c
								     0, // d
								     0), // e
				f: 0,
				g: 0, });
}
