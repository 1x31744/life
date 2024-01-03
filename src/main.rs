use colored::{Colorize};
use std::{thread, time};
use rand::prelude::*;

const GRID_SIZE: (f32, f32) = (118.0, 70.0);

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
            waitTime: 500,
            running: true,
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


        for y in 0..GRID_SIZE.1 as usize{
            let up =  if y > 0 {y - 1} else {GRID_SIZE.1 as usize - 1};
            let down = if y < GRID_SIZE.1 as usize - 1 {y + 1} else { 0 };
            //println!("y axis: {}, {}, {}", y, up, down);

            for x in 0..GRID_SIZE.0 as usize{
                let left = if x > 0 {x - 1} else {GRID_SIZE.0 as usize - 1};
                let right = if x < GRID_SIZE.0 as usize - 1 { x + 1 } else { 0 };
                //println!("x axis: {}, {}, {}", x, right, left);


                //we love circles
                let neighbours 
                = self.grid[y][left] as u8 
                + self.grid[up][left] as u8 
                + self.grid[up][left] as u8 
                + self.grid[up][right] as u8 
                + self.grid[y][right] as u8
                + self.grid[down][right] as u8 
                + self.grid[down][x] as u8 
                + self.grid[down][left] as u8;

                if self.grid[y][x] && (neighbours < 2 || neighbours > 3) {
                    coords.push((y,x));
                }
                else if !self.grid[y][x] && neighbours == 3 {
                    coords.push((y,x));
                }
            }
        }
        for coord in coords {
            self.grid[coord.0][coord.1] ^= true;
        }
    }

 }


fn main(){
    let mut state: GameState = GameState::new();
    let startType: &str = "rand";
    let chanceForLife: u8 = 50;

    if startType == "rand"{
        for x in &mut state.grid{
            for tile in x{
                let randNum = thread_rng().gen_range(0..100);
                if randNum <= chanceForLife {
                    *tile = true
                }
                else{
                    *tile = false
                }
            }
        }
    }

    while state.running{

        //* draw game */
        let noColorOutput: String = state.draw();
        let mut colorOutput = String::from("");
        let mut printingX: bool = true;
        for c in noColorOutput.chars(){
            if c == 'X'{
                //println!("we are printing X");
                if !printingX{
                    print!("{}", colorOutput.as_str().green());
                    colorOutput = String::from("");
                }
                colorOutput.push_str("X ");
                printingX = true;
            }
            else if c == 'O' {
                //println!("we are printing O");
                if printingX{
                    print!("{}", colorOutput.as_str().red());
                    colorOutput = String::from("");
                }
                colorOutput.push_str("O ");
                printingX = false;
            }

            if c == 0xA as char {
                colorOutput.push_str("\n")
            }
        }
        //print final piece as not printed in for loop

        if colorOutput.chars().nth(0).unwrap() == 'X'{
            print!("{}", colorOutput.as_str().red());
        }
        else {
            print!("{}", colorOutput.as_str().green());
        }
        //print!("\n{}", noColorOutput);

        state.updt();
        thread::sleep(time::Duration::from_millis(state.waitTime));
        clearscreen::clear();
    }
}