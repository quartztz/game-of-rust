extern crate osstrtools;

use crate::grid::*;

use sdl2::*;
use sdl2::rect::*;
use sdl2::render::*;

use std::fs::read_dir;

use osstrtools::{OsStrTools, StringSplicer};

// HELPER

pub fn gol_files_in_dir() -> Vec<String> {
	let files = read_dir(".").unwrap()
		.map( |res| res.map(|e| e.file_name().into_string().unwrap() ))
		.collect::<Result<Vec<_>, std::io::Error>>().unwrap();

	let mut ret = vec![];

	for element in files {
		if element.ends_with(".txt") {
			ret.push(element)
		}
	}
	return ret;
}

// MODAL

pub struct ImportModal {
	files: Vec<FileBox>,
	rect: Rect,
	visible: bool,
}

impl ImportModal {
	pub fn build(height: i32, width: i32, scale: i32) -> ImportModal {
		let w = ((width as f32) * 0.4) as i32;
		let h = ((height as f32) * 0.8) as i32;
		let center = (width / 2, height / 2);
		let r = Rect::new(center.0 - (w / 2), center.1 - (h / 2), w as u32, h as u32);
		
		let gol_files = gol_files_in_dir()
			.into_iter()
			.enumerate()
			.map(|(i, f)| {
				FileBox::build(f.to_string(), h/5, w, false, i as i32);
			}).collect::<Vec<FileBox>>();

		ImportModal {
			files: gol_files,
			rect: r,
			visible: false,
		}
	}

	pub fn get_visible(&self) -> bool {
		self.visible
	}

	pub fn toggle(&mut self) {
		self.visible = !self.visible;
	}
}

impl Drawable for ImportModal {
	fn draw(&self, canvas: &mut Canvas<video::Window>) -> () {
		canvas.set_draw_color(BACKGROUND);
		canvas.fill_rect(self.rect);

		canvas.set_draw_color(ALIVE);
		canvas.draw_rect(self.rect);

		for file in &self.files {
			file.draw(canvas);
		}
	}
}

// FILE BOX 

pub struct FileBox {
	file: String,
	height: i32, 
	width: i32, 
	selected: bool,
	index: i32,
}

impl FileBox {
	pub fn build(file: String, height: i32, width: i32, selected: bool, index: i32) -> FileBox {
		FileBox {
			file: file, 
			height: height, 
			width: width, 
			selected: selected, 
			index: index
		}
	}
}

impl Drawable for FileBox {
	fn draw(&self, canvas: &mut Canvas<video::Window>) -> () {
		
	}
}