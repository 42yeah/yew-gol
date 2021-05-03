use rand::Rng;

pub const WORLD_WIDTH: usize = 50;
pub const WORLD_HEIGHT: usize = 50;

#[derive(Debug, Copy, Clone)]
pub enum Cell {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct World {
    pub cells: [[Cell; WORLD_WIDTH]; WORLD_HEIGHT],
}

impl World {
    pub fn new() -> World {
        World {
            cells: [[Cell::Dead; WORLD_WIDTH]; WORLD_HEIGHT],
        }
    }

    pub fn randomize(&mut self) {
        for row in self.cells.iter_mut() {
            for cell in row.iter_mut() {
                if rand::thread_rng().gen::<f32>() < 0.5 {
                    *cell = Cell::Alive;
                } else {
                    *cell = Cell::Dead;
                }
            }
        }
    }

    pub fn get_neighbors(&self, x: i32, y: i32) -> impl Iterator<Item = &Cell> + '_ {
        let mut collected = Vec::<&Cell>::new();
        for dy in -1..2 {
            for dx in -1..2 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let wrapped_y = match dy + y {
                    y if y < 0 => y + WORLD_HEIGHT as i32,
                    y if y >= WORLD_HEIGHT as i32 => y - WORLD_HEIGHT as i32,
                    y => y
                } as usize;
                let wrapped_x = match dx + x {
                    x if x < 0 => x + WORLD_WIDTH as i32,
                    x if x >= WORLD_WIDTH as i32 => x - WORLD_WIDTH as i32,
                    x => x
                } as usize;
                collected.push(&self.cells[wrapped_y][wrapped_x]);
            }
        }
        collected.into_iter()
    }

    pub fn step(&mut self) {
        let mut new_state = self.cells.clone();

        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                let mut alive_neighbors = 0;
                let cell = &self.cells[y][x];
                for cell in self.get_neighbors(x as i32, y as i32) {
                    match cell {
                        Cell::Alive => alive_neighbors += 1,
                        Cell::Dead => {}
                    }
                }

                new_state[y][x] = match (cell, alive_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, x) if x == 2 || x == 3 => Cell::Alive,
                    (Cell::Alive, _) => Cell::Dead,
                    (Cell::Dead, x) if x == 3 => Cell::Alive,
                    _ => cell.clone()
                }
            }
        }
        self.cells = new_state;
    }
}
