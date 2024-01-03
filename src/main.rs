const GRID_SIZE: (f32, f32) = (10.0, 10.0);

struct GameState {
    grid: Vec<Vec<bool>>,
    fps: i32,
    running: bool,
 }

impl GameState{
    pub fn new() -> Self {
        GameState {
        //* creates a grid that is a 2D vector with all tiles having a value of false with size of GRID_SIZE.1 (casted to usize)
        // makes a vector 40 tiles wide and each vector has it's own 40 cells
            grid: vec![vec![false; GRID_SIZE.0 as usize]; GRID_SIZE.1 as usize],
            fps: 1,
            running: true
        }
    }

    pub fn draw(self) -> String{

        println!("{:?}", self.grid);

        let mut currentDraw = String::from("");
        for y in self.grid{
            let mut currentColumnString = String::from("");
            for tile in y {
                currentColumnString.push('X')
            }

            //make a new line
            currentColumnString += "\n";

            currentDraw.push_str(&currentColumnString)
        }
        return currentDraw;
    }
 }


fn main(){
    let state: GameState = GameState::new();
    println!("{}", state.draw());
    

    //* draw game */

}