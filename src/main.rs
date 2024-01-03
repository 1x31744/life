use colored::Colorize;
use std::{thread, time};

const GRID_SIZE: (f32, f32) = (10.0, 10.0);

struct GameState {
    grid: Vec<Vec<bool>>,
    waitTime: u64,
    running: bool,
 }

impl GameState{
    pub fn new() -> Self {
        GameState {
        //* creates a grid that is a 2D vector with all tiles having a value of false with size of GRID_SIZE.1 (casted to usize)
        // makes a vector 40 tiles wide and each vector has it's own 40 cells
            grid: vec![vec![false; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize],
            waitTime: 1,
            running: true
        }
    }

    pub fn draw(&self) -> String{
        let mut currentDraw = String::from("");
        for y in &self.grid{
            let mut currentColumnString = String::from("");
            for tile in y {
                if !*tile{
                    currentColumnString.push_str("X ")
                }
                else{
                    currentColumnString.push_str("O ")
                }
            }

            //make a new line
            currentColumnString += "\n";

            currentDraw.push_str(&currentColumnString)
        }
        return currentDraw;
    }

    pub fn updt(&mut self){
        let mut coords: Vec<(usize, usize)> = vec![]; //stores the co-ordinates of the cells that need to be flipped

        //* draw game */
        println!("{}", self.draw());


        for y in 0..GRID_SIZE.1 as usize{
            let up =  if y > 0 {y-1} else {GRID_SIZE.0 as usize - 1};
            let down = if y < GRID_SIZE.0 as usize - 1 {y + 1} else { 0 };

            for x in 0..GRID_SIZE.0 as usize{
                let right = if x > 0 {x - 1} else {GRID_SIZE.1 as usize-1};
                let left = if x < GRID_SIZE.1 as usize - 1 { x + 1 } else { 0 };


                //we love circles
                let neighbours = self.grid[left][y] as u8 
                + self.grid[left][up] as u8 
                + self.grid[x][up] as u8 
                + self.grid[right][up] as u8 
                + self.grid[right][y] as u8
                + self.grid[right][down] as u8 
                + self.grid[x][down] as u8 
                + self.grid[left][down] as u8;

                if self.grid[x][y] && (neighbours < 2 || neighbours > 3) {
                    coords.push((x,y));
                }
                else if !self.grid[x][y] && neighbours == 3 {
                    coords.push((x,y));
                }
            }
        }
        for coord in coords {
            self.grid[coord.0][coord.1] ^= true;
        }
    }

 }


fn main(){
    let running: bool = true;
    let mut state: GameState = GameState::new();
    let drawState = &state;
    state.grid[5][6] = true;
    state.grid[6][6] = true;
    state.grid[6][7] = true;
    state.grid[6][8] = true;

    while state.running{
        state.updt();
        thread::sleep(time::Duration::from_secs(state.waitTime));
        print!("\x1B[2J\x1B[1;1H");
    }



}