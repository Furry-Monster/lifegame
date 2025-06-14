use lifegame_core::{Cell, World, CELL_ALIVE, CELL_DEAD};
use rand::Rng;
use std::error;

/// Application result type.                                                                 
pub type GameResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, Eq, PartialEq)]
pub enum GameState {
    Run,
    Pause,
    Quit,
}

/// Application.                                                                             
#[derive(Debug)]
pub struct Game {
    /// alive cell probability for random-generated initial map                              
    pub alive_prob: f64,
    /// generation                                                                           
    pub gen: u64,
    /// application state                                                                    
    pub state: GameState,
    /// world size along with x-axis                                                         
    pub nx: usize,
    /// world size along with y-axis                                                         
    pub ny: usize,
    /// the world                                                                            
    pub world: World,
    /// rendering cell index along with x-axis                                               
    pub rendering_ix: usize,
    /// rendering cell index along with y-axis                                               
    pub rendering_iy: usize,
}

fn random_cells(nx: usize, ny: usize, alive_prob: f64) -> Vec<Cell> {
    let mut rng = rand::rng();
    let size = nx * ny;
    (0..size)
        .map(|_| match rng.random_bool(alive_prob) {
            true => CELL_ALIVE,
            false => CELL_DEAD,
        })
        .collect::<Vec<_>>()
}

impl Default for Game {
    fn default() -> Self {
        let (nx, ny) = (120, 60);
        let alive_prob = 0.2;
        let cells = random_cells(nx, ny, alive_prob);
        let world = World::new(nx, ny, &cells).expect("invalid size!");
        Self {
            alive_prob,
            gen: 0,
            state: GameState::Pause,
            nx,
            ny,
            world,
            rendering_ix: 0,
            rendering_iy: 0,
        }
    }
}

impl Game {
    /// Constructs a new instance of [`App`].                                                
    pub fn new() -> Self {
        Self::default()
    }

    pub fn can_reset(&self) -> bool {
        self.state == GameState::Pause
    }

    /// Handles the tick event of the terminal.                                              
    pub fn tick(&mut self) {
        if self.state == GameState::Run {
            self.gen = self.gen.saturating_add(1);
            self.world.next();
        }
    }

    /// Run/pause lifegame                                                                   
    pub fn toggle(&mut self) {
        match self.state {
            GameState::Pause => self.state = GameState::Run,
            GameState::Run => self.state = GameState::Pause,
            _ => (),
        };
    }

    /// Reset lifegame                                                                       
    pub fn reset(&mut self) -> GameResult<()> {
        if self.can_reset() {
            let cells = random_cells(self.nx, self.ny, self.alive_prob);
            self.world = World::new(self.nx, self.ny, &cells)?;
            self.gen = 0;
            self.rendering_ix = 0;
            self.rendering_iy = 0;
        }
        Ok(())
    }

    /// Pan rendering offset along with x-axis                                               
    pub fn pan_x(&mut self, shift: isize) {
        self.rendering_ix = Self::calculate_panned_index(self.rendering_ix, shift, self.nx);
    }

    /// Pan rendering offset along with y-axis                                               
    pub fn pan_y(&mut self, shift: isize) {
        self.rendering_iy = Self::calculate_panned_index(self.rendering_iy, shift, self.ny);
    }

    fn calculate_panned_index(current: usize, shift: isize, upper_limit: usize) -> usize {
        if shift < 0 {
            current.saturating_add_signed(shift)
        } else {
            current.saturating_add_signed(shift).min(upper_limit)
        }
    }

    /// Set running to false to quit the application.                                        
    pub fn quit(&mut self) {
        self.state = GameState::Quit;
    }
}
