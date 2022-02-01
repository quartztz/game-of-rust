use crate::grid::*;

use sdl2::*;
use sdl2::rect::*;
use sdl2::render::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::fs::read_dir;

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
	cur_sel: i32,
	creator: FontCreator,
}

impl ImportModal {
	pub fn build(height: i32, width: i32, scale: i32, fc: FontCreator) -> ImportModal {
		let w = ((width as f32) * 0.4) as i32;
		let h = ((height as f32) * 0.8) as i32;
		let center = (width / 2, height / 2);
		let x = center.0 - (w / 2);
		let y = center.1 - (h / 2);
		let r = Rect::new(x, y, w as u32, h as u32);
		
		let c = gol_files_in_dir().len() as i32;

		let gol_files = gol_files_in_dir()
			.into_iter()
			.enumerate()
			.map(|(i, f)| {
				return FileBox::build(
					f.to_string(), 
					x, 
					y, 
					std::cmp::min(h/5, h/c), 
					w, 
					if i == 0 { true } else { false }, 
					i as i32,
				);
			}).collect::<Vec<FileBox>>();

		ImportModal {
			files: gol_files,
			rect: r,
			visible: false,
			cur_sel: 0,
			creator: fc,
		}
	}
	pub fn get_visible(&self) -> bool {
		self.visible
	}
	pub fn get_chosen_file(&self) -> &str {
		&self.files[self.cur_sel as usize].file
	}
	pub fn toggle(&mut self) {
		self.visible = !self.visible;
	}
	pub fn down(&mut self) {
		self.cur_sel = std::cmp::min(self.files.len() as i32 - 1, self.cur_sel + 1);
		println!("{}", self.files[self.cur_sel as usize].file);
	}
	pub fn up(&mut self) {
		self.cur_sel = std::cmp::max(0, self.cur_sel - 1);
		println!("{}", self.files[self.cur_sel as usize].file);
	}
}

impl Drawable for ImportModal {
	fn draw(&self, canvas: &mut Canvas<video::Window>) -> () {
		canvas.set_draw_color(BACKGROUND);
		canvas.fill_rect(self.rect).unwrap();

		canvas.set_draw_color(ALIVE);
		canvas.draw_rect(self.rect).unwrap();

		for i in 0..self.files.len() {
			self.files[i].draw_with_text(&self.creator, canvas, i == self.cur_sel as usize);
		}
	}
}

// FILE BOX 

pub struct FileBox {
	file: String,
	height: i32, 
	width: i32,
	index: i32,
	x: i32,
	y: i32,
}

impl FileBox {
	pub fn build(file: String, x: i32, y: i32, height: i32, width: i32, selected: bool, index: i32) -> FileBox {
		FileBox {
			file: file, 
			height: height, 
			width: width,
			index: index,
			x: x, 
			y: y + (index) * height,
		}
	}
	pub fn draw_with_text(&self, font_creator: &FontCreator, canvas: &mut Canvas<video::Window>, selected: bool) {
		let rect = Rect::new(self.x, self.y, self.width as u32, self.height as u32);

		let texture = font_creator.create_texture_from_text(&self.file, ALIVE);
		let center = (self.width / 2, self.height / 2);
		let texture_size = (texture.query().width as i32, texture.query().height as i32);
		let target = sdl2::rect::Rect::new(
			self.x + font_creator.scale, 
			self.y + center.1 / 2 + texture_size.1 / 2, 
			((font_creator.scale) * (self.file.len() as i32)) as u32, 
			(font_creator.scale * 2) as u32);

		canvas.set_draw_color(ALIVE);
		if selected { canvas.draw_rect(rect).unwrap(); }
		canvas.copy(&texture, None, Some(target)).unwrap();
	}
}

impl Drawable for FileBox {
	fn draw(&self, canvas: &mut Canvas<video::Window>) -> () {
		let rect = Rect::new(self.x, self.y, self.width as u32, self.height as u32);
		
		canvas.set_draw_color(ALIVE);
		canvas.draw_rect(rect).unwrap();
	}
}

// FONT CREATOR

pub struct FontCreator {
	font: String, // path to the font
	ttf_context: sdl2::ttf::Sdl2TtfContext, // ttf context
	creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>, // texture creator
	scale: i32,
}

impl FontCreator {
	pub fn build(font_path: String, 
							 ttf_context: sdl2::ttf::Sdl2TtfContext, 
							 texture_creator: sdl2::render::TextureCreator<sdl2::video::WindowContext>, 
							 scale: i32) -> FontCreator {
		FontCreator {
			font: font_path,
			ttf_context: ttf_context,
			creator: texture_creator,
			scale: scale
		}
	}
	pub fn create_texture_from_text(&self, text: &str, color: sdl2::pixels::Color) -> Texture {
		let font = self.ttf_context.load_font(&self.font, self.scale as u16).unwrap();
		let surface = font
			.render(text)
			.blended(color)
			.map_err(|e| e.to_string()).unwrap();
		let texture = self.creator
			.create_texture_from_surface(&surface)
			.map_err(|e| e.to_string()).unwrap();

		texture
	}
}
