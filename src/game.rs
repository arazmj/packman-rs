use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

// Constants
const TILE_SIZE: f64 = 16.0;
const MAP_WIDTH: usize = 28;
const MAP_HEIGHT: usize = 31;

// Directions
#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

// Pacman Entity
struct Pacman {
    x: f64,
    y: f64,
    direction: Direction,
    next_direction: Direction,
    speed: f64,
}

impl Pacman {
    fn new() -> Pacman {
        Pacman {
            x: 14.0 * TILE_SIZE, // Starting position (approx)
            y: 23.0 * TILE_SIZE,
            direction: Direction::None,
            next_direction: Direction::None,
            speed: 2.0,
        }
    }

    fn update(&mut self, map: &[u8]) {
        let x_rem = self.x % TILE_SIZE;
        let y_rem = self.y % TILE_SIZE;
        
        // Check if we are at the center of a tile (top-left aligned)
        // Using a small epsilon for float comparison, though with 2.0 speed it should be exact
        let aligned = x_rem.abs() < 0.1 && y_rem.abs() < 0.1;

        if aligned {
            let col = (self.x / TILE_SIZE).round() as usize;
            let row = (self.y / TILE_SIZE).round() as usize;

            // 1. Try to turn to next_direction
            if self.next_direction != Direction::None {
                if self.can_move_to(self.next_direction, col, row, map) {
                    self.direction = self.next_direction;
                    self.next_direction = Direction::None; // Optional: consume input
                }
            }

            // 2. Check if we can continue in current direction
            if !self.can_move_to(self.direction, col, row, map) {
                self.direction = Direction::None; // Stop
            }
        } else {
            // We are between tiles.
            // Allow reversing direction immediately
            if self.next_direction == self.opposite_direction() {
                self.direction = self.next_direction;
                self.next_direction = Direction::None;
            }
        }

        // Move
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Left => self.x -= self.speed,
            Direction::Right => self.x += self.speed,
            Direction::None => {},
        }
    }

    fn can_move_to(&self, dir: Direction, col: usize, row: usize, map: &[u8]) -> bool {
        let (mut next_c, mut next_r) = (col as isize, row as isize);
        
        match dir {
            Direction::Up => next_r -= 1,
            Direction::Down => next_r += 1,
            Direction::Left => next_c -= 1,
            Direction::Right => next_c += 1,
            Direction::None => return false,
        }

        if next_c < 0 || next_r < 0 || next_c >= MAP_WIDTH as isize || next_r >= MAP_HEIGHT as isize {
            return false;
        }

        let tile = map[next_r as usize * MAP_WIDTH + next_c as usize];
        tile != 1 // 1 is wall
    }

    fn opposite_direction(&self) -> Direction {
        match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.save();
        
        let center_x = self.x + TILE_SIZE / 2.0;
        let center_y = self.y + TILE_SIZE / 2.0;
        
        ctx.translate(center_x, center_y).unwrap();
        
        let rotation = match self.direction {
            Direction::Right => 0.0,
            Direction::Down => std::f64::consts::PI / 2.0,
            Direction::Left => std::f64::consts::PI,
            Direction::Up => -std::f64::consts::PI / 2.0,
            Direction::None => 0.0,
        };
        
        ctx.rotate(rotation).unwrap();
        
        // Draw Pacman centered at (0, 0) because we translated
        ctx.begin_path();
        ctx.arc(
            0.0,
            0.0,
            TILE_SIZE / 2.0 - 2.0,
            0.2 * std::f64::consts::PI,
            1.8 * std::f64::consts::PI,
        ).unwrap();
        ctx.line_to(0.0, 0.0);
        ctx.set_fill_style_str("yellow");
        ctx.fill();
        ctx.close_path();
        
        ctx.restore();
    }
}

// Ghost Entity
struct Ghost {
    x: f64,
    y: f64,
    direction: Direction,
    color: String,
    speed: f64,
}

impl Ghost {
    fn new(x: f64, y: f64, color: &str) -> Ghost {
        Ghost {
            x,
            y,
            direction: Direction::Left, // Start moving
            color: color.to_string(),
            speed: 2.0,
        }
    }

    fn update(&mut self, map: &[u8]) {
        let x_rem = self.x % TILE_SIZE;
        let y_rem = self.y % TILE_SIZE;
        let aligned = x_rem.abs() < 0.1 && y_rem.abs() < 0.1;

        if aligned {
            let col = (self.x / TILE_SIZE).round() as usize;
            let row = (self.y / TILE_SIZE).round() as usize;

            // Choose a direction
            // 1. Get all valid moves
            let mut valid_moves = Vec::new();
            for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                // Don't reverse immediately unless stuck
                if dir == self.opposite_direction() {
                    continue;
                }
                if self.can_move_to(dir, col, row, map) {
                    valid_moves.push(dir);
                }
            }

            // If no valid moves (dead end), allow reverse
            if valid_moves.is_empty() {
                let reverse = self.opposite_direction();
                if self.can_move_to(reverse, col, row, map) {
                    valid_moves.push(reverse);
                }
            }

            // Pick random valid move
            if !valid_moves.is_empty() {
                let idx = (js_sys::Math::random() * valid_moves.len() as f64) as usize;
                self.direction = valid_moves[idx];
            } else {
                self.direction = Direction::None;
            }
        }

        // Move
        match self.direction {
            Direction::Up => self.y -= self.speed,
            Direction::Down => self.y += self.speed,
            Direction::Left => self.x -= self.speed,
            Direction::Right => self.x += self.speed,
            Direction::None => {},
        }
    }

    fn can_move_to(&self, dir: Direction, col: usize, row: usize, map: &[u8]) -> bool {
        let (mut next_c, mut next_r) = (col as isize, row as isize);
        match dir {
            Direction::Up => next_r -= 1,
            Direction::Down => next_r += 1,
            Direction::Left => next_c -= 1,
            Direction::Right => next_c += 1,
            Direction::None => return false,
        }

        if next_c < 0 || next_r < 0 || next_c >= MAP_WIDTH as isize || next_r >= MAP_HEIGHT as isize {
            return false;
        }

        let tile = map[next_r as usize * MAP_WIDTH + next_c as usize];
        tile != 1 // 1 is wall
    }

    fn opposite_direction(&self) -> Direction {
        match self.direction {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        let center_x = self.x + TILE_SIZE / 2.0;
        let center_y = self.y + TILE_SIZE / 2.0;

        ctx.begin_path();
        ctx.arc(center_x, center_y, TILE_SIZE / 2.0 - 2.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.set_fill_style_str(&self.color);
        ctx.fill();
        ctx.close_path();
        
        // Eyes
        ctx.set_fill_style_str("white");
        ctx.begin_path();
        ctx.arc(center_x - 4.0, center_y - 2.0, 3.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.arc(center_x + 4.0, center_y - 2.0, 3.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.fill();
        
        ctx.set_fill_style_str("blue");
        ctx.begin_path();
        ctx.arc(center_x - 4.0, center_y - 2.0, 1.5, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.arc(center_x + 4.0, center_y - 2.0, 1.5, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.fill();
    }
}

// Game Struct
#[wasm_bindgen]
pub struct Game {
    width: u32,
    height: u32,
    pacman: Pacman,
    ghosts: Vec<Ghost>,
    map: Vec<u8>,
    score: u32,
    game_over: bool,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let map = vec![
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,
            1,0,1,1,1,1,0,1,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,0,1,
            1,0,1,1,1,1,0,1,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,0,1,
            1,0,1,1,1,1,0,1,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,0,1,
            1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
            1,0,1,1,1,1,0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,0,1,
            1,0,1,1,1,1,0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,0,1,
            1,0,0,0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0,0,0,1,
            1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,1,1,
            1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,1,1,
            1,1,1,1,1,1,0,1,1,0,0,0,0,0,0,0,0,0,0,1,1,0,1,1,1,1,1,1,
            1,1,1,1,1,1,0,1,1,0,1,1,1,0,0,1,1,1,0,1,1,0,1,1,1,1,1,1,
            1,1,1,1,1,1,0,1,1,0,1,0,0,0,0,0,0,1,0,1,1,0,1,1,1,1,1,1,
            0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,
            1,1,1,1,1,1,0,1,1,0,1,0,0,0,0,0,0,1,0,1,1,0,1,1,1,1,1,1,
            1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1,
            1,1,1,1,1,1,0,1,1,0,0,0,0,0,0,0,0,0,0,1,1,0,1,1,1,1,1,1,
            1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1,
            1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1,
            1,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,1,
            1,0,1,1,1,1,0,1,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,0,1,
            1,0,1,1,1,1,0,1,1,1,1,1,0,1,1,0,1,1,1,1,1,0,1,1,1,1,0,1,
            1,0,0,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,0,1,
            1,1,1,0,1,1,0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,1,0,1,1,1,
            1,1,1,0,1,1,0,1,1,0,1,1,1,1,1,1,1,1,0,1,1,0,1,1,0,1,1,1,
            1,0,0,0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0,0,0,1,
            1,0,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,
            1,0,1,1,1,1,1,1,1,1,1,1,0,1,1,0,1,1,1,1,1,1,1,1,1,1,0,1,
            1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,
            1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
        ];

        let ghosts = vec![
            Ghost::new(13.0 * TILE_SIZE, 11.0 * TILE_SIZE, "red"),
            Ghost::new(14.0 * TILE_SIZE, 11.0 * TILE_SIZE, "pink"),
            Ghost::new(13.0 * TILE_SIZE, 13.0 * TILE_SIZE, "cyan"),
            Ghost::new(14.0 * TILE_SIZE, 13.0 * TILE_SIZE, "orange"),
        ];

        Game {
            width: MAP_WIDTH as u32 * TILE_SIZE as u32,
            height: MAP_HEIGHT as u32 * TILE_SIZE as u32,
            pacman: Pacman::new(),
            ghosts,
            map,
            score: 0,
            game_over: false,
        }
    }

    fn check_pellet_collision(&mut self) {
        let center_x = self.pacman.x + TILE_SIZE / 2.0;
        let center_y = self.pacman.y + TILE_SIZE / 2.0;
        
        let col = (center_x / TILE_SIZE) as usize;
        let row = (center_y / TILE_SIZE) as usize;

        if col < MAP_WIDTH && row < MAP_HEIGHT {
            let index = row * MAP_WIDTH + col;
            if self.map[index] == 0 {
                self.map[index] = 2; // 2 = Empty (eaten)
                self.score += 10;
            }
        }
    }

    pub fn tick(&mut self) {
        if self.game_over {
            return;
        }

        self.pacman.update(&self.map);
        self.check_pellet_collision();
        
        for ghost in &mut self.ghosts {
            ghost.update(&self.map);
            
            // Check collision with Pacman
            let dist_sq = (self.pacman.x - ghost.x).powi(2) + (self.pacman.y - ghost.y).powi(2);
            if dist_sq < (TILE_SIZE * 0.8).powi(2) {
                self.game_over = true;
            }
        }
    }

    pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
        // Clear screen
        ctx.set_fill_style_str("black");
        ctx.fill_rect(0.0, 0.0, self.width as f64, self.height as f64);

        // Draw Map
        for row in 0..MAP_HEIGHT {
            for col in 0..MAP_WIDTH {
                let tile = self.map[row * MAP_WIDTH + col];
                if tile == 1 {
                    ctx.set_fill_style_str("blue");
                    ctx.fill_rect(
                        col as f64 * TILE_SIZE,
                        row as f64 * TILE_SIZE,
                        TILE_SIZE,
                        TILE_SIZE,
                    );
                } else if tile == 0 {
                    // Draw pellet
                    ctx.set_fill_style_str("white");
                    ctx.fill_rect(
                        col as f64 * TILE_SIZE + TILE_SIZE / 2.0 - 1.0,
                        row as f64 * TILE_SIZE + TILE_SIZE / 2.0 - 1.0,
                        2.0,
                        2.0,
                    );
                }
            }
        }

        // Draw Pacman
        self.pacman.draw(ctx);
        
        // Draw Ghosts
        for ghost in &self.ghosts {
            ghost.draw(ctx);
        }
        
        // Draw Score
        ctx.set_fill_style_str("white");
        ctx.set_font("16px Courier New");
        ctx.fill_text(&format!("Score: {}", self.score), 10.0, 20.0).unwrap();
        
        if self.game_over {
            ctx.set_fill_style_str("red");
            ctx.set_font("40px Courier New");
            ctx.fill_text("GAME OVER", self.width as f64 / 2.0 - 100.0, self.height as f64 / 2.0).unwrap();
        }
    }

    pub fn set_direction(&mut self, dir_code: u32) {
        self.pacman.next_direction = match dir_code {
            38 => Direction::Up,    // ArrowUp
            40 => Direction::Down,  // ArrowDown
            37 => Direction::Left,  // ArrowLeft
            39 => Direction::Right, // ArrowRight
            _ => self.pacman.next_direction,
        };
    }
}
