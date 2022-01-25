#![allow(unused_must_use)]

use sdl2::*;
use sdl2::rect::*;
use sdl2::render::*;

use rand::Rng;
use std::cmp;
use std::fs;

pub const BACKGROUND: pixels::Color = pixels::Color::RGB(0, 0, 0);
pub const DEAD: pixels::Color = pixels::Color::RGB(0, 0x80, 0);
pub const ALIVE: pixels::Color = pixels::Color::RGB(0, 0xff, 0);

pub const P: f32 = 0.75;

pub trait Drawable {
	fn draw(&self, canvas: &mut Canvas<video::Window>) -> () { }
}

// HELPER

pub fn open_file_lines(file: &str) -> Vec<String> {
	return fs::read_to_string(file)
		.expect("something went wrong")
		.split("\n")
		.map(|s| s.to_string())
		.collect();
}

// CELL

pub struct Cell {
	x: i32,
	y: i32,
	rect: Rect,
	alive: bool
}

impl Cell {
	pub fn build(x: i32, y: i32, scale: i32, p: f32) -> Cell {
		let mut rng_gen = rand::thread_rng();
		Cell {
			x: x,
			y: y,
			rect: Rect::new(x * scale, y * scale, scale as u32, scale as u32),
			alive: if rng_gen.gen::<f32>() > p { true } else { false } ,
		}
	}
	pub fn tick(&self, neighbours: i32) -> Cell {
		let next: bool;
		if self.alive {
			next = neighbours == 2 || neighbours == 3;
		} else {
			next = neighbours == 3;
		}

		Cell {
			x: self.x, 
			y: self.y, 
			rect: self.rect,
			alive: next,
		}
	}

	pub fn update(&self) -> Cell {
		Cell {
			x: self.x, 
			y: self.y,
			rect: self.rect,
			alive: !self.alive,
		}
	}

	pub fn copy(&self) -> Cell {
		Cell {
			x: self.x, 
			y: self.y, 
			rect: self.rect, 
			alive: self.alive,
		}
	}
}

impl Drawable for Cell {
	fn draw(&self, canvas: &mut Canvas<video::Window>) -> () {
		if !self.alive {
			canvas.set_draw_color(DEAD);
			canvas.draw_rect(self.rect);
		} else {
			canvas.set_draw_color(ALIVE);
			canvas.fill_rect(self.rect);
		}
	}
}

// GRID

pub struct Grid {
	cells: Vec<Vec<Cell>>,
	max_x: i32, 
	max_y: i32,
	scale: i32
}

impl Grid {
	pub fn build(height: i32, width: i32, scale: i32) -> Grid {
		let mut cells: Vec<Vec<Cell>> = vec![];

		for i in 0..height/scale {
			let mut line: Vec<Cell> = vec![];
			for j in 0..width/scale {
				line.push(Cell::build(j as i32, i as i32, scale, P));
			}
			cells.push(line);
		}

		Grid {
			cells: cells,
			max_x: width / scale,
			max_y: height / scale,
			scale: scale,
		}
	}

	pub fn build_empty(height: i32, width: i32, scale: i32) -> Grid {
		let mut cells: Vec<Vec<Cell>> = vec![];

		for i in 0..height/scale {
			let mut line: Vec<Cell> = vec![];
			for j in 0..width/scale {
				line.push(Cell::build(j as i32, i as i32, scale, 1.0));
			}
			cells.push(line);
		}

		Grid {
			cells: cells,
			max_x: width / scale,
			max_y: height / scale,
			scale: scale,
		}
	}

	pub fn build_from_file(height: i32, width: i32, scale: i32, file: &str) -> Grid {
		let mut cells: Vec<Vec<Cell>> = vec![];
		let lines: Vec<String> = open_file_lines(file);

		assert_eq!(lines.len() as i32, height/scale, "number of lines doesn't match canvas size");
		assert_eq!(lines[0].len() as i32, width/scale, "length of line doesn't match canvas size");

		for i in 0..height/scale {
			let mut line: Vec<Cell> = vec![];
			for j in 0..width/scale {
				let current_ch = lines[i as usize].chars().nth(j as usize).unwrap();
				if current_ch == '0' {
					line.push(Cell::build(j as i32, i as i32, scale, 1.0));
				} else if current_ch == '1' {
					line.push(Cell::build(j as i32, i as i32, scale, 0.0));
				} else {
					println!("unrecognized character, assumed dead");
					line.push(Cell::build(j as i32, i as i32, scale, 1.0));
				}
			}
			cells.push(line);
		}

		Grid {
			cells: cells,
			max_x: width / scale,
			max_y: height / scale,
			scale: scale,
		}
	}

	fn get_alive_neighbours(&self, cell: &Cell) -> i32 {
		let mut count: i32 = 0;

		let min_y = cmp::max(cell.y - 1, 0);
		let max_y = cmp::min(cell.y + 1, self.max_y - 1);
		let min_x = cmp::max(cell.x - 1, 0);
		let max_x = cmp::min(cell.x + 1, self.max_x - 1);

		let neighbours: Vec<(i32, i32)> = vec![
			(min_x, cell.y),
			(min_x, max_y),
			(cell.x, max_y),
			(max_x, max_y),
			(max_x, cell.y),
			(max_x, min_y),
			(cell.x, min_y),
			(min_x, min_y),
		];

		for pos in neighbours {
			if self.cells[pos.1 as usize][pos.0 as usize].alive {
				count += 1;
			}
		}
		return count;

	}

	pub fn update(&mut self, e: &sdl2::EventPump) -> bool {
		if e.mouse_state().left() {
			let pos: (i32, i32)  = (
				e.mouse_state().x() / self.scale, 
				e.mouse_state().y() / self.scale
			);
			let mut new_cells: Vec<Vec<Cell>> = vec![];
			for line in &self.cells {
				let mut new_line: Vec<Cell> = vec![];
				for cell in line {
					if cell.y == pos.1 && cell.x == pos.0 {
						new_line.push(cell.update());
					} else {
						new_line.push(cell.copy());
					}
				}
				new_cells.push(new_line);
			}
			self.cells = new_cells;
			return true;
		}
		return false;
	}

	pub fn tick(&mut self) {
		// init new array
		let mut new_cells = vec![];

		// loop over old array
		for line in &self.cells {
			// create new line
			let mut new_line = vec![];

			// loop over old line
			for cell in line {
				new_line.push(cell.tick(self.get_alive_neighbours(cell)));
			}
			new_cells.push(new_line);
		}
		self.cells = new_cells;
	}
}

impl Drawable for Grid {
	fn draw(&self, canvas: &mut Canvas<video::Window>) -> () {
		for line in &self.cells {
			for cell in line {
				cell.draw(canvas);
			}
		}
	}
}