/*
 * Copyright (c) 2014 Arcterus
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */ 

pub use self::target::*;
use runtime::iter;
use runtime::str::*;

#[path = "arch/target/console.rs"]
pub mod target;

pub enum Color {
	Black      = 0,
	Blue       = 1,
	Green      = 2,
	Cyan       = 3,
	Red        = 4,
	Magenta    = 5,
	Brown      = 6,
	LightGray  = 7,
	DarkGray   = 8,
	LightBlue  = 9,
	LightGreen = 10,
	LightCyan  = 11,
	LightRed   = 12,
	Pink       = 13,
	Yellow     = 14,
	White      = 15
}

pub static BACKGROUND_COLOR: Color = DarkGray;
pub static FOREGROUND_COLOR: Color = LightCyan;

pub static SCREEN_SIZE: uint = MAX_ROW * MAX_COLUMN;

#[packed]
struct ScreenChar {
	char: u8,
	attr: u8
}

pub type Screen = [ScreenChar, ..SCREEN_SIZE];

static mut SCREEN: *mut Screen = SCREEN_ADDR as *mut Screen;

static mut row: uint = 0;
static mut col: uint = 0;

#[inline]
pub fn print(msg: &str) {
	color_print(msg, FOREGROUND_COLOR, BACKGROUND_COLOR);
}

#[inline]
pub fn println(msg: &str) {
	color_println(msg, FOREGROUND_COLOR, BACKGROUND_COLOR);
}

#[inline]
pub fn clear_screen() {
	color_clear_screen(BACKGROUND_COLOR);
}

pub fn color_println(msg: &str, foreground: Color, background: Color) {
	color_print(msg, foreground, background);
	print_byte('\n' as u8, foreground, background);
	unsafe {
		move_cursor(row, col);
	}
}

pub fn color_print(msg: &str, foreground: Color, background: Color) {
	unsafe {
		msg.each_byte(|byte| {
			print_byte(byte, foreground, background);
			true
		});
		move_cursor(row, col);
	}
}

#[inline]
pub unsafe fn print_bytes(msg: *u8) {
	color_print_bytes(msg, FOREGROUND_COLOR, BACKGROUND_COLOR);
}

pub unsafe fn color_print_bytes(msg: *u8, foreground: Color, background: Color) {
	let mut msg = msg;
	while *msg != 0 {
		print_byte(*msg, foreground, background);
		msg = (msg as uint + 1) as *u8;
	}
	move_cursor(row, col);
}

#[inline]
fn print_byte(byte: u8, foreground: Color, background: Color) {
	unsafe {
		match byte {
			0x0a /* newline */ => add_line(background),
			0x0d /* carriage return */ => col = 0,
			0x08 /* backspace */ => {
				if col == 0 && row != 0 {
					col = MAX_COLUMN - 1;
					row -= 1;
				} else if col != 0 {
					col -= 1;
				}
			}
			byte => {
				let pos = row * MAX_COLUMN + col;
				(*SCREEN)[pos].char = byte as u8;
				(*SCREEN)[pos].attr = ((background as u8) << 4) + (foreground as u8);
				col += 1;
				if col == MAX_COLUMN {
					add_line(background);
				}
			}
		}
	}
}

pub fn color_clear_screen(background: Color) {
	unsafe {
		iter::range(0, MAX_ROW,|i| {
			clear_line(*i, background);
		});
		row = 0;
		col = 0;
		move_cursor(0, 0);
	}
}

fn clear_line(_row: uint, background: Color) {
	unsafe {
		let c = col;
		let r = row;
		col = 0;
		row = _row;
		clear_rem_line(background);
		row = r;
		col = c;
	}
}

fn clear_rem_line(background: Color) {
	unsafe {
		let rpos = row * MAX_COLUMN;
		iter::range(col, MAX_COLUMN, |i| {
			let pos = rpos + *i;
			(*SCREEN)[pos].char = ' ' as u8;
			(*SCREEN)[pos].attr = ((background as u8) << 4) + (FOREGROUND_COLOR as u8);
		})
	}
}

fn add_line(background: Color) {
	clear_rem_line(background);
	unsafe {
		col = 0;
		row += 1;
		if row == MAX_ROW {
			row -= 1;
			shift_rows_up();
		}
	}
}

fn shift_rows_up() {
	unsafe {
		iter::range(1, MAX_ROW, |r| {
			let fposr = *r * MAX_COLUMN;
			let tposr = fposr - MAX_COLUMN;
			iter::range(0, MAX_COLUMN, |c| {
				let tpos = tposr + *c;
				let fpos = fposr + *r;
				(*SCREEN)[tpos].char = (*SCREEN)[fpos].char;
				(*SCREEN)[tpos].attr = (*SCREEN)[fpos].attr;
			});
		});
	}
	clear_line(MAX_ROW - 1, BACKGROUND_COLOR);
}
