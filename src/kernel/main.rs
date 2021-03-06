/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 
#[crate_id(name = "kRnel",
           vers = "0.0.1",
           author = "Arcterus",
           license = "MPL v2.0")];


#[allow(ctypes)];
#[no_std];
#[feature(globs)];
#[feature(asm)];


pub use drivers::io::console;
pub use target::reset::*;

#[path = "arch/target"]
mod target {
	pub mod reset;
}

#[path = "../runtime/mod.rs"]
pub mod runtime;

#[path = "../drivers"]
mod drivers {
	pub mod io {
		pub mod console;
	}
}

pub mod error;

#[no_mangle]
#[start]
pub fn main() {
	console::clear_screen();
	console::print("iiiiiiiiiiiiiiiiiiiiiiiiiii\niiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii\x08\x08\x08\x08\x08test");
	console::println("");
	error::panic("End of kernel");
}
